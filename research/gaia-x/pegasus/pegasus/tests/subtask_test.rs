//
//! Copyright 2020 Alibaba Group Holding Limited.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use pegasus::api::{CorrelatedSubTask, Count, Map, Sink, HasAny, Limit};
use pegasus::JobConf;
use std::collections::HashMap;

#[test]
fn apply_x_map_flatmap_count_x_test() {
    let mut conf = JobConf::new("apply_x_map_flatmap_count_x_test");
    conf.set_workers(2);
    let num = 1000u32;
    let mut result = pegasus::run(conf, move || {
        let index = pegasus::get_current_worker().index;
        move |input, output| {
            let src = if index == 0 { input.input_from(0..num) } else { input.input_from(num..2 * num) }?;

            src.apply(|sub| {
                sub.map(|i| Ok(i + 1))?
                    .repartition(|x| Ok(*x as u64))
                    .flat_map(|i| Ok(0..i))?
                    .count()
            })?
            .sink_into(output)
        }
    })
    .expect("build job failure");

    let mut count = 0;
    while let Some(Ok(d)) = result.next() {
        if count < 10 {
            println!("{}: {}=>{}", count, d.0, d.1);
        }
        let cnt = Some(d.0)
            .into_iter()
            .map(|i| i + 1)
            .flat_map(|i| (0..i))
            .count() as u64;
        assert_eq!(cnt, d.1);
        count += 1;
    }
    assert_eq!(count, num * 2);
}

fn apply_x_flatmap_flatmap_count_x_test(workers: u32) {
    let name = format!("apply_x_flatmap_flatmap_count_x_{}_test", workers);
    let mut conf = JobConf::new(name);
    conf.set_workers(workers);
    let num = 100u32;
    let mut result = pegasus::run(conf, move || {
        let index = pegasus::get_current_worker().index;
        let source = (num * index)..(index + 1u32) * num;
        move |input, output| {
            let src = input.input_from(source)?;
            src.apply(|sub| {
                sub.flat_map(|i| Ok(0..i + 1))?
                    .repartition(|x| Ok(*x as u64))
                    .flat_map(|i| Ok(0..i))?
                    .count()
            })?
            .sink_into(output)
        }
    })
    .expect("build job failure");

    let mut count = 0;
    while let Some(Ok((p, cnt))) = result.next() {
        let expected = (0..p + 1).flat_map(|i| (0..i)).count() as u64;
        assert_eq!(expected, cnt, "{} expected cnt = {}", p, expected);
        count += 1;
    }
    assert_eq!(count, num * workers);
}

#[test]
fn apply_x_flatmap_flatmap_count_x_2_test() {
    apply_x_flatmap_flatmap_count_x_test(2)
}

#[test]
fn apply_x_flatmap_flatmap_count_x_3_test() {
    apply_x_flatmap_flatmap_count_x_test(3)
}

#[test]
fn apply_x_flatmap_flatmap_count_x_4_test() {
    apply_x_flatmap_flatmap_count_x_test(4)
}

#[test]
fn apply_x_flatmap_flatmap_count_x_5_test() {
    apply_x_flatmap_flatmap_count_x_test(5)
}

#[test]
fn apply_x_flatmap_flatmap_count_x_6_test() {
    apply_x_flatmap_flatmap_count_x_test(6)
}

#[test]
fn apply_x_flatmap_flatmap_count_x_7_test() {
    apply_x_flatmap_flatmap_count_x_test(7)
}

#[test]
fn apply_x_flatmap_flatmap_count_x_8_test() {
    apply_x_flatmap_flatmap_count_x_test(8)
}

#[test]
fn apply_x_flatmap_flatmap_agg_map_count_x_test() {
    let mut conf = JobConf::new("apply_x_flatmap_flatmap_agg_map_count_x_test");
    conf.set_workers(2);
    let num = 100u32;
    let mut result = pegasus::run(conf, move || {
        let index = pegasus::get_current_worker().index;
        move |input, output| {
            let src = if index == 0 { input.input_from(0..num) } else { input.input_from(num..2 * num) }?;
            src.apply(|sub| {
                sub.flat_map(|i| Ok(0..i + 1))?
                    .repartition(|x| Ok(*x as u64))
                    .flat_map(|i| Ok(0..i))?
                    .aggregate()
                    .map(|x| Ok(x + 1))?
                    .count()
            })?
            .sink_into(output)
        }
    })
    .expect("build job failure");

    let mut count = 0;
    while let Some(Ok((p, cnt))) = result.next() {
        let expected = (0..p + 1).flat_map(|i| (0..i)).count() as u64;
        assert_eq!(expected, cnt, "{} expected cnt = {}", p, expected);
        count += 1;
    }
    assert_eq!(count, num * 2);
}

#[test]
fn apply_x_flatmap_any_x_test() {
    let mut conf = JobConf::new("apply_x_flatmap_any_x_test");
    conf.set_workers(2);
    let mut result = pegasus::run(conf, || {
        |input, output| {
            input
                .input_from(0..1000u32)?
                .apply(|sub| {
                    sub.repartition(|x| Ok(*x as u64))
                        .flat_map(|i| Ok(std::iter::repeat(i)))?
                        .any()
                })?
                .sink_into(output)
        }
    })
        .expect("build job failure");

    let mut count = 0;
    while let Some(Ok(d)) = result.next() {
        assert!(d.0 < 1000);
        assert!(d.1);
        count += 1;
    }

    assert_eq!(count, 2000);
}

fn read_test_data(filename: &str) -> HashMap<u64, Vec<u64>> {
    use std::io::{BufReader, BufRead};
    use std::fs::File;

    let mut map: HashMap<u64, Vec<u64>> = HashMap::new();
    let reader = BufReader::new(File::open(filename).unwrap());
    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut data = line.split('|');
        let data_str = data.next().unwrap();
        let v = data_str.parse().expect(&format!("{:?} cannot parse int", data_str));
        let nbr: Vec<u64> = data.map(|s|s.parse().unwrap()).collect();
        map.entry(v)
            .or_insert_with(Vec::new)
            .extend(nbr.into_iter());
    }
    map
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref MAP1: HashMap<u64, Vec<u64>> = read_test_data("tests/data/bug1_data.csv");
    pub static ref MAP2: HashMap<u64, Vec<u64>> = read_test_data("tests/data/bug2_data.csv");
}

#[test]
fn apply_flatmap_limit_unexpected_results1() {
    use std::sync::Arc;

    let mut conf = JobConf::new("apply_flatmap_limit_unexpected_results");
    let expected_cnt = MAP1.len() as u64;

    conf.set_workers(4);
    let mut result = pegasus::run(conf, move || {
        let index = pegasus::get_current_worker().index;
        move |input, output| {
            let src = if index == 0 {
                input.input_from(MAP1.keys().map(|x| *x))?
            } else {
                input.input_from(vec![].into_iter())?
            };
            src
                .apply(|sub| {
                    sub.repartition(|v| Ok(*v))
                        .flat_map(|v| {
                            Ok(MAP1.get(&v).unwrap().iter()
                                .map(|x| *x))
                        })?
                        .limit(1)?
                        .count()
                })?
                .filter_map(|(v, cnt)| if cnt == 0 { Ok(None) } else { Ok(Some(v)) })?
                .count()?
                .sink_into(output)
        }
    })
        .expect("build job failure");

    while let Some(Ok(cnt)) = result.next() {
        assert_eq!(cnt, expected_cnt);
    }
}

#[test]
fn apply_flatmap_limit_unexpected_results2() {
    let mut conf = JobConf::new("apply_flatmap_limit_unexpected_results2");
    let src_v: u64 = 1 << 56 | 17592186044810;
    let expected_cnt: u64 = 2000;

    conf.set_workers(2);
    let mut result = pegasus::run(conf, move || {
        let index = pegasus::get_current_worker().index;

        move |input, output| {
            let src = if index == 0 {
                input.input_from(MAP2.get(&src_v).unwrap().iter().map(|x| *x))?
            } else {
                input.input_from(vec![].into_iter())?
            };
            src
                .repartition(|v| Ok(*v))
                .flat_map(move |v| {
                    Ok(
                        MAP2.get(&v).unwrap().iter().map(|x| *x)
                    )
                })?
                .limit(expected_cnt as u32)?
                .apply(move |sub| {
                    sub.repartition(|v| Ok(*v))
                        .flat_map(|v|Ok(MAP2.get(&v).unwrap().iter().map(|x| *x)))?
                        .repartition(|v| Ok(*v))
                        .flat_map(|v| Ok(MAP2.get(&v).unwrap().iter().map(|x| *x)))?
                        .limit(1)?
                        .count()
                })?
                .filter_map(|(v, cnt)| if cnt == 0 { Ok(None) } else { Ok(Some(v)) })?
                .count()?
                .sink_into(output)
        }
    })
        .expect("build job failure");

    while let Some(Ok(cnt)) = result.next() {
        assert_eq!(cnt, expected_cnt);
    }
}