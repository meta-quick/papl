import paply


def test_new_cedar():
    engine = paply.PaplCedarEngine()
    assert engine != None

def test_policy():
    engine = paply.PaplCedarEngine()
    POLICY_SRC = """
    permit(principal == User::"alice", action == Action::"view", resource == File::"931");
    """
    engine.add_policy(POLICY_SRC)
    action = """
    Action::"view"

    """
    user = """
    User::"alice"

    """
    resource = """
    File::"931"

    """

    resource1 = """
    File::"93"

    """

    decision = engine.decide_request(user, action, resource,"{}")

    assert decision == "ALLOW"

    decision = engine.decide_request(user, action, resource1,"{}")

    assert decision == "DENY"