package poller;

import jitlib.rs.jit.jitlib.ByteList;
import poller.util.UnhandledStateException;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.channels.Channel;
import java.nio.channels.SelectionKey;
import java.nio.channels.SocketChannel;

public final class PollableSocketChannel implements Channel {
    private final PollLoop loop;
    private final SocketChannel channel;
    private SelectionKey key;
    private boolean isOpen = true;

    private PollableSocketChannel(PollLoop loop, SocketChannel channel, SelectionKey key) {
        this.loop = loop;
        this.channel = channel;
        this.key = key;
    }

    public static PollableSocketChannel open(PollLoop loop, SocketChannel channel) {
        try {
            channel.configureBlocking(false);

            return new PollableSocketChannel(loop, channel, null);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public PollableTask<ByteList> read() {
        return new PollableTask<>(loop) {
            @Override
            public ByteList internalPoll(TaskHandle rootTask) {
                try {
                    switch (state) {
                        case 0:
                            key = channel.register(loop.selector, SelectionKey.OP_READ);
                            loop.bind(key, rootTask);
                            state = 1;
                        case 1:
                            assertOpen();

                            var list = new ByteList();
                            var buf = ByteBuffer.allocate(1000);
                            while (true) {
                                var bytesRead = channel.read(buf);
                                System.out.println(bytesRead);

                                if (bytesRead == -1) {
//                                    throw new RuntimeException("channel closed");

                                    // Channel is closed
                                    isOpen = false;

                                    key.cancel();
                                    loop.unBind(key, rootTask);
                                    state = -1;
                                    return list;
                                }
                                if (bytesRead == 0)
                                    break;

                                list.extend(buf.array(), 0, buf.position());

                                buf = ByteBuffer.allocate(1000);
                            }
                            if (list.size() > 0) {
                                // Read successful. Channel is not closed, but task is no longer pollable
                                loop.unBind(key, rootTask);
                                state = -1;
                                return list;
                            } else {
                                state = 1;
                                return null;
                            }
                        default:
                            throw new UnhandledStateException();
                    }
                } catch (Exception e) {
                    throw new RuntimeException(e);
                }
            }
        };
    }

    @Override
    public void close() throws IOException {
        channel.close();
    }

    @Override
    public boolean isOpen() {
        return isOpen;
    }

    private void assertOpen() {
        if (!isOpen()) throw new RuntimeException("channel is closed");
    }
}
