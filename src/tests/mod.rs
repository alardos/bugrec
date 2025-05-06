use std::{vec, dbg, fs::{File, self}, io::{BufRead, Read}, path::PathBuf};

use crate::http_server::{self, http_param::HttpParamValue, http_request::{self, HttpRequest, parse_multipart_boundary, multipart_distribution, parse_multipart_parts}};

// mod merge_test;

#[test]
fn parse_params() {
   assert!(http_request::parse_params("GET /api/records/get?id=1").contains_key("id"));
   assert_eq!(http_server::http_request::parse_params("GET /api/records/get?id=1").get("id").unwrap(), "1")
}

#[test]
fn parse_complex_params_test() {
    let result = http_request::parse_complex_params("GET /api/records/get?id=1");
    assert!(result.contains_key("id"));
    assert!(*result.get("id").unwrap() == HttpParamValue::Singular("1".to_string()));

    let result = http_request::parse_complex_params("GET /api/records/get?id=1,2");
    assert!(result.contains_key("id"));
    assert!(*result.get("id").unwrap() == HttpParamValue::List(vec!["1".to_string(),"2".to_string()]))
}

#[test]
fn get_boundary_from_multipart() {
    let file = fs::File::open("resources/test/multipart_parser_test_file");
    let mut buff = [0;10024]; 
    file.unwrap().read(&mut buff).unwrap();
    let boundary = dbg!(parse_multipart_boundary(&buff));
    assert_eq!("--------------------------387377177625789407279784", boundary.unwrap());
}

#[test]
fn multipart_distribution_test() {
    let file = fs::File::open("resources/test/multipart_parser_test_file");
    let mut buff = [0;10024]; 
    file.unwrap().read(&mut buff).unwrap();
    let boundary = "--------------------------387377177625789407279784".as_bytes();
    let dist = multipart_distribution(&buff, boundary);
    let x: Vec<usize> = vec![];
    assert_eq!(dist.boundary_indexes,x)
}

#[test]
fn multipart_distribution_test_basic() {
    let buff = vec![1,0,0,3,43,10,21,43,3,0,0,32,43,33]; 
    let boundary = vec![0,0];
    let dist = multipart_distribution(&buff, &boundary);
    let x: Vec<usize> = vec![9];
    assert_eq!(dist.boundary_indexes,x)
}

#[test]
fn find_subsequence_test() {
    let find_subsequence = |haystack: &[u8], needle: &[u8]| haystack.windows(needle.len()).position(|window| window == needle);
    let input = vec![1,2,3,4,5,6,7,8,9];
    assert_eq!(find_subsequence(&input,&vec![3,4]),Some(2));
    assert_eq!(find_subsequence(&input,&vec![4,4]),None);

}

#[test]
fn parse_multipart_test() {
    let file = fs::File::open("resources/test/multipart_parser_test_file");
    let mut buff = [0;10024]; 
    file.unwrap().read(&mut buff).unwrap();
    let boundary = "--------------------------387377177625789407279784".as_bytes();
    let distribution = multipart_distribution(&buff, boundary);
    let files: Vec<PathBuf> = parse_multipart_parts(&buff, distribution);
}


