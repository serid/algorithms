import jitlib.rs.jit.jitlib.ByteList;
import poller.*;
import poller.util.UnhandledStateException;
import jitlib.rs.jit.jitlib.Unit;

import java.net.InetSocketAddress;
import java.nio.channels.Selector;
import java.nio.charset.StandardCharsets;

public class Main {
    public static void main(String[] args) {
        try {
            var loop = new PollLoop(Selector.open());

            var srv = PollableSocketServer.open(loop, new InetSocketAddress("localhost", 8080));

            var task = new PollableTask<Unit>(loop) {
                private PollableTask<PollableSocketChannel> channelTask;
                private PollableTask<ByteList> readingTask;
                private PollableSocketChannel channel;

                @Override
                public Unit internalPoll(TaskHandle rootTask) {
                    switch (state) {
                        case 0:
                            channelTask = srv.accept();
                            state = 1;
                        case 1:
                            var ch = channelTask.poll(rootTask);
                            if (ch == null) return null;
                            System.out.println(ch);
                            channel = ch;
                            state = 2;
                        case 2:
                            System.out.println("Reading channel");
                            readingTask = channel.read();
                            state = 3;
                        case 3:
                            var list = readingTask.poll(rootTask);
                            if (list == null) return null;

                            // Handle channel closing
                            if (!channel.isOpen()) {
                                // Accept another channel (this should be done concurrently)
                                state = 0;
                                return poll(rootTask);
                            }

                            // TODO: implement HTTP protocol switch
                            var str = new String(list.toArray(), StandardCharsets.UTF_8);

                            System.out.println(str);

                            state = 2;
                            return poll(rootTask);
//                            return Unit.instance;

                        default:
                            throw new UnhandledStateException();
                    }
                }
            };

            loop.run(task);
            loop.close();

//            var s = Selector.open();
//
//            var serverChannel = ServerSocketChannel.open();
//            serverChannel.configureBlocking(false);
//            var serverKey = serverChannel.register(s, SelectionKey.OP_ACCEPT);
//            serverChannel.bind(new InetSocketAddress("localhost", 8080));
//
//            var connections = new ArrayList<Pair<SocketChannel, SelectionKey>>();
//
//            while (String.valueOf(true).equals("true")) {
//                var readyChannels = s.select();
//                var selectedKeys = s.selectedKeys();
//
//                selectedKeys.stream().forEach((key) -> {
//                    try {
//                        if (key == serverKey) {
//                            var ch = serverChannel.accept();
//                            if (ch != null) {
//                                ch.configureBlocking(false);
//                                connections.add(new Pair<>(ch, ch.register(s, SelectionKey.OP_READ)));
//                            }
//                        } else connections.stream().forEach(pair -> {
//                            try {
//                                if (key == pair.b) {
//                                    var buf = ByteBuffer.allocate(10);
//                                    var bytesRead = pair.a.read(buf);
//                                    while (buf.hasRemaining())
//                                        System.out.print(buf.get());
//                                    System.out.println();
//                                }
//                            } catch (Exception e) {
//                                throw new RuntimeException(e);
//                            }
//                        });
//                    } catch (Exception e) {
//                        throw new RuntimeException(e);
//                    }
//                });
//            }
//
//            s.close();
        } catch (Exception e) {
            unwrap(e).printStackTrace();
        }
    }

    public static Throwable unwrap(Throwable e) {
        while (e.getCause() != null) {
            e = e.getCause();
        }
        return e;
    }
}
