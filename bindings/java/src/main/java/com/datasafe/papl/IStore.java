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
public interface IStore extends AutoCloseable {
    public long save(String key,String value,String version,long stamp);
    public void delete(String key);
    public String get(String key);
    public String version(String key);
    public String[] versionValue(String key);
    public String[] allKeysLE(long stamp);
    public String[] allKeysBE(long stamp);
    public String[] AllKeysBEPageable(long stamp,long page,long size);
    public long EvictLE(long stamp);
    public long EvictBE(long stamp);
}
