package com.datasafe.papl;

import java.util.concurrent.locks.Lock;

/**
 * @author gaosg
 */
public class DeferLock implements AutoCloseable {
    private final Lock lock;

    public DeferLock(Lock lock) {
        this.lock = lock;
        this.lock.lock();
    }

    @Override
    public void close() {
        lock.unlock();
    }
}
