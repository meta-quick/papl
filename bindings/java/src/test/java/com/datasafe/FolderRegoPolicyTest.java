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
import com.datasafe.policy.FolderRegoPolicy;
import com.datasafe.policy.ResultSet;
import junit.framework.TestCase;

public class FolderRegoPolicyTest extends TestCase {
    public void test_policy(){
        IStore store = new MemoryStore();
        FolderRegoPolicy policy = new FolderRegoPolicy(store);

        policy.prepare("a/b/c.txt","package policy\ndefault allow := false\n");
        policy.prepare("a/b/","package policy\ndefault allow := true\n");

        ResultSet r = policy.eval("data.policy.allow","a/b/c.txt","{}","{}",false);
        System.out.println(r);
        r = policy.eval("data.policy.allow","a/b/","{}","{}",false);
        System.out.println(r);
        r = policy.eval("data.policy.list","a/b/","{}","{}",false);
        System.out.println(r);
    }

    public void test_foldEval(){
        IStore store = new MemoryStore();
        FolderRegoPolicy policy = new FolderRegoPolicy(store);
        policy.prepare("a/b/c.txt","package policy\ndefault allow := false\n");
        policy.prepare("a/b/","package policy\ndefault list := true\n");
        ResultSet r = policy.foldEval("data.policy.list","a/b/c.txt","{}","{}");
        System.out.println(r.toExpression()[0].value);
        r = policy.foldEval("data.isRecursive","a/b/c.txt","{}","{}");
        System.out.println(r.toExpression()[0].value);
    }

    public void test_storage() {
        try(IStore store = new FileStore("aaa/aa.db")) {
            store.save("a/b/c","{THIS IS DEMO}");
            System.out.println(store.get("a/b/c"));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
