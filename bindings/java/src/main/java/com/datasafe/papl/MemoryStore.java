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

/**
 * @author gaosg
 */
public class MemoryStore implements IStore {

    private long handle = 0;

    public MemoryStore() {
        handle = Engine.nativeNewMemoryStore();
    }

    @Override
    public void save(String key,String value){
        if (handle == 0 || handle == -1){
            return;
        }
        Engine.nativeStoreSave(this.handle,key,value);
    }

    @Override
    public void delete(String key) {
        if (handle == 0 || handle == -1){
            return;
        }
        Engine.nativeStoreDelete(handle,key);
    }

    @Override
    public String get(String key){
        if (handle == 0 || handle == -1){
            return null;
        }
        return Engine.nativeStoreGet(handle,key);
    }

    @Override
    public void close() throws Exception {
        if (handle == 0 || handle == -1){
            return;
        }
        Engine.nativeCloseStore(handle);
    }
}
