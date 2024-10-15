/*
 * Copyright 2024 brian <gao.brian@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.datasafe.papl;

import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

/**
 * @author gaosg
 */
public class MemoryStore implements IStore {

    private long handle = 0;
    private final Lock lock = new ReentrantLock();

    public MemoryStore() {
        handle = Engine.nativeNewMemoryStore();
    }

    @Override
    public long save(String key,String value,String version,long stamp){
        if (handle == 0 || handle == -1){
            return -1;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeStoreSave(this.handle, key, value, version, stamp);
        }
    }

    @Override
    public void delete(String key) {
        if (handle == 0 || handle == -1){
            return;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            Engine.nativeStoreDelete(handle, key);
        }
    }

    @Override
    public String get(String key){
        if (handle == 0 || handle == -1){
            return null;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeStoreGet(handle, key);
        }
    }

    @Override
    public String version(String key) {
        if (handle == 0 || handle == -1){
            return null;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeStoreGetVersion(handle, key);
        }
    }

    @Override
    public String[] versionValue(String key) {
        if (handle == 0 || handle == -1){
            return null;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            String[] value = Engine.nativeStoreGetVersionValue(handle, key);
            if (value != null && value.length > 0) {
                return value;
            }
            return null;
        }
    }

    @Override
    public String[] allKeysLE(long stamp) {
        if (handle == 0 || handle == -1){
            return null;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeAllKeysLE(handle, stamp);
        }
    }

    @Override
    public String[] allKeysBE(long stamp) {
        if (handle == 0 || handle == -1){
            return null;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeAllKeysBE(handle, stamp);
        }
    }

    @Override
    public String[] AllKeysBEPageable(long stamp, long page, long size) {
        if (handle == 0 || handle == -1){
            return null;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeAllKeysBEPageable(handle, stamp, page, size);
        }
    }

    @Override
    public long EvictLE(long stamp) {
        if (handle == 0 || handle == -1){
            return 0;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeEvictLE(handle, stamp);
        }
    }

    @Override
    public long EvictBE(long stamp) {
        if (handle == 0 || handle == -1){
            return 0;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            return Engine.nativeEvictBE(handle, stamp);
        }
    }

    @Override
    public void close() throws Exception {
        if (handle == 0 || handle == -1){
            return;
        }
        try (DeferLock deferLock = new DeferLock(lock)) {
            Engine.nativeCloseStore(handle);
        }
    }
}
