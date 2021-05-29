package poller;

import poller.util.UnhandledStateException;

import java.io.IOException;
import java.net.SocketAddress;
import java.nio.channels.SelectionKey;
import java.nio.channels.ServerSocketChannel;

public final class PollableSocketServer {
    private final PollLoop loop;
    private final ServerSocketChannel server;
    private SelectionKey key;

    private PollableSocketServer(PollLoop loop, ServerSocketChannel server, SelectionKey key) {
        this.loop = loop;
        this.server = server;
        this.key = key;
    }

    public static PollableSocketServer open(PollLoop loop, SocketAddress address) {
        try {
            ServerSocketChannel server = loop.selector.provider().openServerSocketChannel();
            server.configureBlocking(false);
            server.bind(address);

            return new PollableSocketServer(loop, server, null);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public PollableTask<PollableSocketChannel> accept() {
        return new PollableTask<>(loop) {
            @Override
            public PollableSocketChannel internalPoll(TaskHandle rootTask) {
                try {
                    switch (state) {
                        case 0:
                            key = server.register(loop.selector, SelectionKey.OP_ACCEPT);
                            loop.bind(key, rootTask);
                            state = 1;
                        case 1:
                            var ch = server.accept();
                            if (ch == null) return null;

                            key.cancel();
                            loop.unBind(key, rootTask);
                            state = -1;
                            return PollableSocketChannel.open(loop, ch);
                        default:
                            throw new UnhandledStateException();
                    }
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        };
    }
}
