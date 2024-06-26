import paply

def test_new_engine():
    engine = paply.PaplRegoEngine()

def test_rego():
    engine = paply.PaplRegoEngine()

    #add bundles
    engine.add_bundles(["examples"])

    #add input
    engine.add_input("examples/input.json")

    #add input json
    engine.add_input_json("""
    {
        "name": "bob",
        "age": 20,
        "friends": ["alice"]
    }
    """)

    # add_policy_from_file
    engine.add_policy_from_file("examples/example.rego")

    #test add_policy_from_string
    engine.add_policy_from_string("hello","""
    package hello
    default allow = false
    allow {
        input.name == "bob"
    }
    """)

    #test add_data
    engine.add_data("examples/data.json")

    #test add_data_from_string
    engine.add_data_from_string("{}")

    #test clear_data
    engine.clear_data()

    #test #test clear_data
    result = engine.eval_query("data.hello.allow")
    print(result)


    #test eval
    result = engine.eval_rule("data.hello.allow")
    print(result)