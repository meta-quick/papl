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

import com.datasafe.papl.FileStore;
import com.datasafe.papl.IStore;
import com.datasafe.papl.MemoryStore;
import junit.framework.TestCase;
import org.junit.Assert;

import java.net.URL;
import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.util.Base64;

public class MemoryStoreTest extends TestCase {
    public void test_memory_store() throws Exception {
        MemoryStore store = new MemoryStore();

        store.save("hello","world","1",1);
        String world = store.get("hello");
        System.out.println(world);
        System.out.println(store.version("hello"));

        System.out.println(store.versionValue("hello")[0]);
        System.out.println(store.versionValue("hello")[1]);

        store.delete("hello");
        world = store.get("hello");
        System.out.println(world);
    }

    public void test_sqlite_store_allkeys() throws Exception {
        IStore store = new FileStore("test.db");
        store.save("hello","world","1",1);
        store.save("hello1","world","1",2);
        store.save("hello2","world","1",3);

        System.out.println(store.allKeysLE(2).length);
        System.out.println(store.allKeysBE(2).length);
    }

    public void test_sqlite_store() throws Exception {
        IStore store = new FileStore("test.db");
        store.save("hello","world","1",1);
        String world = store.get("hello");

        System.out.println(world);

        for (int i = 0; i < 100; i++) {
            long ret = store.save("hello","world"+i,String.valueOf(i),i);
            System.out.println(">"+ret+":"+store.version("hello"));
        }

        store.close();
    }

    public void test_base(){
        //BASE64 decode

        String x =  URLDecoder.decode("aHR0cDovL2xvY2FsaG9zdDo1MTczL3NoYWRvd2RyaXZlL2RvYy1maWxlL3ZpZXc%2FZG9jUmVzb3VyY2VJZD0zNTgwNjA2NTEwNTMwNTcmZmlsZVBhdGg95paH5qGjL09LUi54bHN4JmZ1bGxmaWxlbmFtZT1PS1IueGxzeA%3D%3D");

        byte[]  xx = Base64.getDecoder().decode(x.getBytes());

        System.out.println(new String(xx, StandardCharsets.UTF_8));

    }
}
