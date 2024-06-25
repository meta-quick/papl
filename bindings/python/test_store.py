import paply


def test_new_store():
    store = paply.PaplStore("","memory")
    assert store != None

def test_new_store_with_file():
    store = paply.PaplStore("test.db","file")
    assert store != None
    store.close()
    #remove test.db file
    import os
    os.remove("test.db")

def test_save_get():
    store = paply.PaplStore("","memory")
    store.save("test", "test001","v11000")
    assert store.get("test") == "test001"
    try:
        store.get("test1")
        assert False
    except:
        pass

def test_save_get_with_version():
    store = paply.PaplStore("","memory")
    store.save("test", "test001","v11000")
    vv = store.value_with_version("test")
    val,ver = vv
    assert val == "test001"
    assert ver == "v11000"
