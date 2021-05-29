package poller;

public abstract class PollableTask<T> {
    protected final PollLoop loop;
    protected byte state = 0;

    protected PollableTask(PollLoop loop) {
        this.loop = loop;
    }

    /**
     * @return task result or <pre>null</pre> if task is pending
     */
    public T poll(TaskHandle rootTask) {
        if (state == -1)
            throw new RuntimeException("poll should not be called after it returns non-null in previous call");
        return internalPoll(rootTask);
    }

    public abstract T internalPoll(TaskHandle rootTask);
}
