// Copyright 2024 brian <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use jni::objects::{JClass, JObject, JString,JObjectArray};
use jni::sys::{jlong, jobjectArray, jstring};
use jni::JNIEnv;
use papl::*;

fn throw_err<T>(mut env: JNIEnv, mut f: impl FnMut(&mut JNIEnv) -> Result<T>) -> Result<T> {
    match f(&mut env) {
        Ok(val) => Ok(val),
        Err(err) => {
            env.throw(err.to_string())?;
            Err(err)
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeNewCedarEngine(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = CedarEngine::new();
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCloseCedarEngine(
    _env : JNIEnv,
    _class: JClass,
    _engine: jlong,
) -> jlong {
    println!("Closing Cedar Engine {}", _engine);
    unsafe {
        let _engine = Box::from_raw(_engine as *mut CedarEngine);
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeNewRegoEngine(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = RegoEngine::new(false);
    Box::into_raw(Box::new(engine)) as jlong
}


#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCloseRegoEngine(
    _env : JNIEnv,
    _class: JClass,
    _engine: jlong,
) -> jlong {
    println!("Closing Rego Engine {}", _engine);
    unsafe {
        let _engine = Box::from_raw(_engine as *mut RegoEngine);
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddPolicy<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    path: JString<'local>,
    rego: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let path: String = env.get_string(&path)?.into();
        let rego: String = env.get_string(&rego)?.into();
        engine.add_policy_from_string(path,rego)?;
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddBundles<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    bundles: JObjectArray<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let len = env.get_array_length(&bundles).unwrap();

        let mut local_bundles = Vec::<String>::with_capacity(len as usize);
        for i in 0..len {
            let bundle: JString = env.get_object_array_element(&bundles,i).unwrap().into();
            let bundle_path: String = env.get_string(&bundle)?.into();
            local_bundles.push(bundle_path);
        }
        engine.add_bundles(&local_bundles.as_slice())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddInput<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    input: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let input: String = env.get_string(&input)?.into();
        println!("Adding input: {}", input);
        engine.add_input(Some(input))
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddInputJSON<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    json: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let json: String = env.get_string(&json)?.into();
        engine.add_input_json(Some(json))
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    path: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let path: String = env.get_string(&path)?.into();
        engine.add_data(Some(path))
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddStringData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    data: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let data: String = env.get_string(&data)?.into();
        engine.add_data_from_string(data)
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddPolicyFile<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    path: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let path: String = env.get_string(&path)?.into();
        engine.add_policy_from_file(path)
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoClearData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
) {
    let _ = throw_err(env, |_env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        engine.clear_data();
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoEvalQuery<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    query: JString<'local>,
) -> jstring {
    let res = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let query: String = env.get_string(&query)?.into();
        let results = engine.eval_query(query, false)?;
        let output = env.new_string(serde_json::to_string(&results)?)?;
        Ok(output.into_raw())
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCedarAddPolicy<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    policy: JString<'local>,
) {
    let _res = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut CedarEngine) };
        let policy: String = env.get_string(&policy)?.into();
        let _results = engine.add_policy(policy)?;
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCedarAddEntity<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    enitity: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut CedarEngine) };
        let enitity: String = env.get_string(&enitity)?.into();
        let _results = engine.add_entity(enitity.as_str());
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCedarAuthorize<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    principal: JString<'local>,
    action: JString<'local>,
    resource: JString<'local>,
    json_context: JString<'local>,
) -> jstring{
    let res = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut CedarEngine) };

        let principal: String = env.get_string(&principal)?.into();
        let action: String = env.get_string(&action)?.into();
        let resource: String = env.get_string(&resource)?.into();
        let json_context: String = env.get_string(&json_context)?.into();

        let results = engine.authorize_request(principal, action, resource, json_context);

        let decision = results.decision();
        let decision = match decision {
            cedar_policy::Decision::Deny => "DENY",
            cedar_policy::Decision::Allow => "ALLOW",
        };


        let decision = env.new_string(&decision).unwrap();
        Ok(decision.into_raw())
    });

    match res {
        Ok(val) => {
           val
        },
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeNewStore<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
) -> jlong {
    let result = throw_err(env, |env| {
        let path: String = env.get_string(&path)?.into();
        let sqlite = SqliteStore::new(&path);

        match sqlite {
            Ok(store) => {
                let store = Box::into_raw(Box::new(store)) as jlong;
                Ok(store)
            },
            Err(_) => {
                Ok(-1)
            }
        }
    });

    match result {
        Ok(val) => val,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeNewMemoryStore<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jlong {
    let result = throw_err(env, |_env| {
        let memory = SqliteStore::new_in_memory();

        match memory {
            Ok(store) => {
                let store = Box::into_raw(Box::new(store)) as jlong;
                Ok(store)
            },
            Err(_) => {
                Ok(-1)
            }
        }
    });

    match result {
        Ok(val) => val,
        Err(_) =>0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeStoreSave<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    key: JString<'local>,
    value: JString<'local>,
    version: JString<'local>,
    stamp: jlong,
) -> jlong {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };

        let key: String = env.get_string(&key)?.into();
        let value: String = env.get_string(&value)?.into();
        let version: String = env.get_string(&version)?.into();
        let stamp = stamp as i64;

        let results = store.save(key, value, version, stamp);
        match results {
            Ok(val) => {
                Ok(val as jlong)
            },
            Err(_) => {
                Ok(0)
            }
        }
    });
    match res {
        Ok(val) => val,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeStoreGet<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    key: JString<'local>,
) -> jstring {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let key: String = env.get_string(&key)?.into();
        let results = store.get(key);

        match results {
            Ok(val) => {
                let val = env.new_string(val).unwrap();
                Ok(val.into_raw())
            },
            Err(_) => {
                Ok(JObject::null().into_raw())
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeStoreDelete<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    key: JString<'local>,
) -> jlong {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let key: String = env.get_string(&key)?.into();
        let results = store.delete(key);

        match results {
            Ok(val) => {
                Ok(val as jlong)
            },
            Err(_) => {
                Ok(0)
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCloseStore<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
) {
    unsafe {
        let _engine = Box::from_raw(store_ptr as *mut SqliteStore);
        _engine.close();
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeStoreGetVersionValue<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    key: JString<'local>,
) -> jobjectArray {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let key: String = env.get_string(&key)?.into();
        let results = store.version_value(key);

        match results {
            Ok((policy,version)) => {
                let _policy = env.new_string(policy).unwrap();
                let _version = env.new_string(version).unwrap();
                let class = env.find_class("java/lang/String").expect("Failed to find class");
                let object_array = env.new_object_array(2,class, JObject::null()).expect( "Failed to create array");

                env.set_object_array_element(&object_array, 0, _policy).expect("Failed to set object array element");
                env.set_object_array_element(&object_array, 1, _version).expect("Failed to set object array element");

                Ok(object_array.into_raw())
            },
            Err(e) => {
                Ok(JObject::null().into_raw())
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeStoreGetVersion<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    key: JString<'local>,
) -> jstring {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let key: String = env.get_string(&key)?.into();
        let results = store.version(key);

        match results {
            Ok(val) => {
                let val = env.new_string(val).unwrap();
                Ok(val.into_raw())
            },
            Err(_) => {
                Ok(JObject::null().into_raw())
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeAllKeysBE<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    stamp: jlong,
) -> jobjectArray {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let stamp = stamp as i64;

        let results = store.all_keys_be(stamp);

        match results {
            Ok(keys) => {
                let size = keys.len() as i32;
                let class = env.find_class("java/lang/String").expect("Failed to find class");
                let key_array = env.new_object_array(size,class, JObject::null()).expect( "Failed to create array");

                //for each key in keys
                let mut index = 0;
                for elem in keys.iter() {
                    let key = env.new_string(elem).unwrap();
                    env.set_object_array_element(&key_array,index, key).expect("Failed to set object array element");
                    index += 1;
                }
                Ok(key_array.into_raw())
            },
            Err(e) => {
                println!("{}",e.to_string());
                Ok(JObject::null().into_raw())
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeAllKeysLE<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    stamp: jlong,
) -> jobjectArray {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let stamp = stamp as i64;
        let results = store.all_keys_le(stamp);

        match results {
            Ok(keys) => {
                let size = keys.len() as i32;
                let class = env.find_class("java/lang/String").expect("Failed to find class");
                let key_array = env.new_object_array(size,class, JObject::null()).expect( "Failed to create array");

                //for each key in keys
                let mut index = 0;
                for elem in keys.iter() {
                    let key = env.new_string(elem).unwrap();
                    env.set_object_array_element(&key_array,index, key).expect("Failed to set object array element");
                    index += 1;
                }
                Ok(key_array.into_raw())
            },
            Err(e) => {
                println!("{}",e.to_string());
                Ok(JObject::null().into_raw())
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeAllKeysBEPageable<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    stamp: jlong,
    page: jlong,
    size: jlong,
) -> jobjectArray {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let stamp = stamp as i64;
        let page = page as i64;
        let szie = size as i64;
        let results = store.all_keys_be_pageable(stamp, page, size);

        match results {
            Ok(keys) => {
                let size = keys.len() as i32;
                let class = env.find_class("java/lang/String").expect("Failed to find class");
                let key_array = env.new_object_array(size,class, JObject::null()).expect( "Failed to create array");

                //for each key in keys
                let mut index = 0;
                for elem in keys.iter() {
                    let key = env.new_string(elem).unwrap();
                    env.set_object_array_element(&key_array,index, key).expect("Failed to set object array element");
                    index += 1;
                }
                Ok(key_array.into_raw())
            },
            Err(e) => {
                println!("{}",e.to_string());
                Ok(JObject::null().into_raw())
            }
        }
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeEvictBE<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    stamp: jlong,
) -> jlong {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let stamp = stamp as i64;
        let results = store.evict_be(stamp);
        match results {
            Ok(keys) => {
                Ok(keys)
            },
            Err(e) => {
                Ok(0)
            }
        }
    });

    match res {
        Ok(val) => val as jlong,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeEvictLE<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    store_ptr: jlong,
    stamp: jlong,
) -> jlong {
    let res = throw_err(env, |env| {
        let store = unsafe { &mut *(store_ptr as *mut SqliteStore) };
        let stamp = stamp as i64;
        let results = store.evict_le(stamp);
        match results {
            Ok(keys) => {
                Ok(keys)
            },
            Err(e) => {
                Ok(0)
            }
        }
    });

    match res {
        Ok(val) => val as jlong,
        Err(_) => 0,
    }
}
