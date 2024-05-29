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

package com.datasafe;

import com.datasafe.papl.MemoryStore;
import junit.framework.Assert;
import junit.framework.TestCase;

public class MemoryStoreTest extends TestCase {
    public void test_memory_store() throws Exception {
        MemoryStore store = new MemoryStore();

        store.save("hello","world","1");
        String world = store.get("hello");
        System.out.println(world);
        System.out.println(store.version("hello"));

        System.out.println(store.versionValue("hello")[1]);
        System.out.println(store.versionValue("hello")[0]);

        store.delete("hello");
        world = store.get("hello");
        System.out.println(world);
    }
}
