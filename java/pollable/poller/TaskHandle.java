package poller;

import jitlib.rs.jit.jitlib.Unit;

public final class TaskHandle {
    public final PollableTask<Unit> task;

    public TaskHandle(PollableTask<Unit> task) {
        this.task = task;
    }
}
