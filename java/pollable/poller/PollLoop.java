package poller;

import jitlib.rs.jit.jitlib.Unit;

import java.io.Closeable;
import java.io.IOException;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.ConcurrentHashMap;

public final class PollLoop implements Closeable {
    public final Selector selector;
    private final ArrayBlockingQueue<TaskHandle> queue = new ArrayBlockingQueue<>(100);
    private final ConcurrentHashMap<SelectionKey, TaskHandle> ioBinds = new ConcurrentHashMap<>();

    public PollLoop(Selector selector) {
        this.selector = selector;
    }

    void bind(SelectionKey key, TaskHandle task) {
        var old = ioBinds.put(key, task);
        if (old != null) throw new RuntimeException();
    }

    void unBind(SelectionKey key, TaskHandle task) {
        var success = ioBinds.remove(key, task);
        if (!success) throw new RuntimeException();
    }

    public final void run(PollableTask<Unit> entryPoint) {
        try {
            queue.add(new TaskHandle(entryPoint));

            //noinspection InfiniteLoopStatement
            while (true) {
                System.out.println("iteration");

                // process active tasks
                while (!queue.isEmpty()) {
                    var handle = queue.remove();
                    handle.task.poll(handle);
                }

                var readyChannels = selector.select();

                if (readyChannels == 0)
                    throw new RuntimeException();

                var selectedKeys = selector.selectedKeys();

                var iterator = selectedKeys.iterator();
                while (iterator.hasNext()) {
                    var key = iterator.next();
                    iterator.remove();

                    System.out.println("Ready ops for channel:" + key.readyOps());

                    var v = ioBinds.get(key);
                    if (v == null) System.out.println("[warn] received an event with no bound task");
                    else queue.add(v);
                }
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void close() throws IOException {
        selector.close();
    }
}
