use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::cmp::{max, min};

pub fn vip_scheduler(s: &[u32], e: &[u32]) -> Result<u32> {
    if s.len() != e.len() {
        return Err(anyhow!("Start and end time arrays do not match in length!"));
    }

    // Create zipped sorted array of VIP times
    let mut vip_times: Vec<(u32, u32)> = Vec::new();
    for (start_time, end_time) in s.iter().zip(e.iter()) {
        vip_times.push((*start_time, *end_time));
    }
    vip_times.sort_by_key(|x| x.0);

    let mut performance_count = 0;
    let mut overlap_boundary = vip_times[0];

    let mut skip_until: Option<usize> = None;

    for (index, _vip_time) in vip_times.iter().enumerate() {
        // Workaround since we cannot rewrite the index
        if let Some(i) = skip_until {
            if index < i {
                continue;
            }
        }

        let mut next_index = index + 1;

        while next_index < vip_times.len() {
            let next_element = &vip_times[next_index];

            if (overlap_boundary.0..=overlap_boundary.1).contains(&next_element.0)
                || (overlap_boundary.0..=overlap_boundary.1).contains(&next_element.1)
            {
                // Set new overlap boundaries
                overlap_boundary.0 = max(overlap_boundary.0, next_element.0);
                overlap_boundary.1 = min(overlap_boundary.1, next_element.1);

                if overlap_boundary.0 > overlap_boundary.1 {
                    break;
                }
                next_index += 1;
            } else {
                break;
            }
        }

        performance_count += 1;
        skip_until = Some(next_index);
        if next_index < vip_times.len() {
            overlap_boundary = vip_times[next_index];
        }
    }

    Ok(performance_count)
}

// Only used for test harness, silence dead code warning
#[allow(dead_code)]
#[derive(Deserialize)]
struct TestCase {
    name: String,
    s: Vec<u32>,
    e: Vec<u32>,
    result: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_scheduler_tcs() {
        let tcs_str = include_str!("../q2_test_cases.json");
        let tcs: Vec<TestCase> = serde_json::from_str(&tcs_str).expect("Invalid TC JSON file!");

        for tc in tcs {
            assert_eq!(
                vip_scheduler(tc.s.as_slice(), tc.e.as_slice()).unwrap(),
                tc.result,
                "Test case {} failed!",
                tc.name
            );
        }
    }
}
