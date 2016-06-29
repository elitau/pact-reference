#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use pact_matching::match_request;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
#[ignore]
fn missing_params() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Queries are not the same - elephant is missing",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John&elephant=missing",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=Fred",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn matches_with_equals_in_the_query_value() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Queries are equivalent",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "options=delete.topic.enable=true&broker=1",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "options=delete.topic.enable%3Dtrue&broker=1",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn same_parameter_multiple_times() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Queries are the same - multiple values are in same order",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=hippo&animal=elephant&hippo=Fred",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&hippo=Fred&animal=hippo&animal=elephant",
          "headers": {}
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn unexpected_param() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Queries are not the same - elephant is not expected",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=Fred&elephant=unexpected",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn different_order() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Queries are the same but in different key order",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "hippo=John&alligator=Mary",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn different_params() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Queries are not the same - hippo is Fred instead of John",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=Fred",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn matches() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Queries are the same",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn same_parameter_different_values() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Queries are not the same - animals are alligator, hippo versus alligator, elephant",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=hippo",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=elephant",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn same_parameter_multiple_times_in_different_order() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Queries are not the same - values are in different order",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=hippo&animal=elephant",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "animal=hippo&animal=alligator&animal=elephant",
          "headers": {}
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn trailing_ampersand() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Trailing amperands can be ignored",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John&",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}