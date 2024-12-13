use crate::api::interface::ip::{IPAdd, IPAddV6Query, IPSub, IPSubV6Query};
use axum::extract::Query;
use std::iter::zip;

pub async fn calculate_ip_add(ip: Query<IPAdd>) -> String {
    let ip_from_split_vec = ip.from.split('.').collect::<Vec<&str>>();
    let ip_key_split_vec = ip.key.split('.').collect::<Vec<&str>>();

    let new_ip = zip(ip_from_split_vec, ip_key_split_vec)
        .map(|(from, key)| {
            let from = from.parse::<u16>().unwrap();
            let key = key.parse::<u16>().unwrap();
            (from + key) % 256
        })
        .map(|sum| sum.to_string())
        .collect::<Vec<String>>()
        .join(".");
    new_ip
}

pub async fn calculate_ip_sub(ip: Query<IPSub>) -> String {
    let ip_from_split_vec = ip.from.split('.').collect::<Vec<&str>>();
    let ip_to_split_vec = ip.to.split('.').collect::<Vec<&str>>();

    let new_ip = zip(ip_from_split_vec, ip_to_split_vec)
        .map(|(from, to)| {
            let from = from.parse::<i16>().unwrap();
            let to = to.parse::<i16>().unwrap();
            return if to - from < 0 {
                to - from + 256
            } else {
                to - from
            };
        })
        .map(|sum| sum.to_string())
        .collect::<Vec<String>>()
        .join(".");
    new_ip
}

#[derive(Debug)]
struct IPAddV6 {
    from: Vec<u16>,
    key: Vec<u16>,
}
impl IPAddV6 {
    const IPV6_LEN: usize = 8;

    fn new(from: &str, key: &str) -> Self {
        let from = Self::parse_ipv6(from);
        let key = Self::parse_ipv6(key);
        IPAddV6 { from, key }
    }

    fn parse_ipv6(ip_v6: &str) -> Vec<u16> {
        let mut ret = Vec::with_capacity(Self::IPV6_LEN);
        let is_abbreviated = ip_v6.contains("::");
        if is_abbreviated {
            let ip_v6 = ip_v6.split("::").collect::<Vec<&str>>();
            let ip_v6_1 = ip_v6[0].split(":").collect::<Vec<&str>>();
            let ip_v6_2 = ip_v6[1].split(":").collect::<Vec<&str>>();

            for ip in &ip_v6_1 {
                if ip == &"" {
                    ret.push(0);
                    continue;
                }
                let ipe = u16::from_str_radix(ip, 16);
                if let Ok(ipe) = ipe {
                    ret.push(ipe)
                } else {
                    panic!("Error: {}", ipe.unwrap_err());
                }
            }
            for _ in 0..(Self::IPV6_LEN - ip_v6_1.len() - ip_v6_2.len()) {
                ret.push(0);
            }
            for ip in &ip_v6_2 {
                if ip == &"" {
                    ret.push(0);
                    continue;
                }
                let ipe = u16::from_str_radix(ip, 16);
                if let Ok(ipe) = ipe {
                    ret.push(ipe)
                } else {
                    panic!("Error: {}", ipe.unwrap_err());
                }
            }
        } else {
            let ip_v6 = ip_v6.split(":");
            for ip in ip_v6 {
                let ipe = u16::from_str_radix(ip, 16);
                if let Ok(ipe) = ipe {
                    ret.push(ipe)
                } else {
                    panic!("Error: {}", ipe.unwrap_err());
                }
            }
        }
        ret
    }

    fn xor(self) -> Vec<u16> {
        let mut ret = Vec::with_capacity(Self::IPV6_LEN);
        for (from, key) in self.from.iter().zip(self.key.iter()) {
            ret.push(from ^ key);
        }
        ret
    }
}

pub async fn calculate_ipv6_add(ip_add_v6: Query<IPAddV6Query>) -> String {
    println!("--- Input ---");
    println!("From: {}", ip_add_v6.from);
    println!("Key: {}", ip_add_v6.key);
    let ip_v6 = IPAddV6::new(&ip_add_v6.from, &ip_add_v6.key);
    println!("model: {:?}", ip_v6);
    let ans = ip_v6.xor();
    // ansで0が連続している部分を省略して、::に置き換える
    let mut ret = String::new();
    let mut is_abbreviated = false;
    for (i, ip) in ans.iter().enumerate() {
        if *ip == 0 {
            if !is_abbreviated {
                ret.push_str(":");
                is_abbreviated = true;
            }
        } else {
            ret.push_str(&format!("{:x}", ip));
            if i != IPAddV6::IPV6_LEN - 1 {
                ret.push(':');
            }
        }
    }
    println!("Ans: {}", ret);
    ret
}

#[derive(Debug)]
struct IpSubV6 {
    from: Vec<u16>,
    to: Vec<u16>,
}
impl IpSubV6 {
    const IPV6_LEN: usize = 8;

    fn new(from: &str, to: &str) -> Self {
        let from = Self::parse_ipv6(from);
        let to = Self::parse_ipv6(to);
        IpSubV6 { from, to }
    }

    fn parse_ipv6(ip_v6: &str) -> Vec<u16> {
        let mut ret = Vec::with_capacity(Self::IPV6_LEN);
        let is_abbreviated = ip_v6.contains("::");
        if is_abbreviated {
            let ip_v6 = ip_v6.split("::").collect::<Vec<&str>>();
            let ip_v6_1 = ip_v6[0].split(":").collect::<Vec<&str>>();
            let ip_v6_2 = ip_v6[1].split(":").collect::<Vec<&str>>();

            for ip in &ip_v6_1 {
                if ip == &"" {
                    ret.push(0);
                    continue;
                }
                let ipe = u16::from_str_radix(ip, 16);
                if let Ok(ipe) = ipe {
                    ret.push(ipe)
                } else {
                    panic!("Error: {}", ipe.unwrap_err());
                }
            }
            for _ in 0..(Self::IPV6_LEN - ip_v6_1.len() - ip_v6_2.len()) {
                ret.push(0);
            }
            for ip in &ip_v6_2 {
                if ip == &"" {
                    ret.push(0);
                    continue;
                }
                let ipe = u16::from_str_radix(ip, 16);
                if let Ok(ipe) = ipe {
                    ret.push(ipe)
                } else {
                    panic!("Error: {}", ipe.unwrap_err());
                }
            }
        } else {
            let ip_v6 = ip_v6.split(":");
            for ip in ip_v6 {
                let ipe = u16::from_str_radix(ip, 16);
                if let Ok(ipe) = ipe {
                    ret.push(ipe)
                } else {
                    panic!("Error: {}", ipe.unwrap_err());
                }
            }
        }
        ret
    }

    fn to_string(self) -> String {
        let mut ret = String::new();
        for (i, ip) in self.from.iter().enumerate() {
            ret.push_str(&format!("{:x}", ip));
            if i != Self::IPV6_LEN - 1 {
                ret.push(':');
            }
        }
        ret
    }

    fn xor(self) -> Vec<u16> {
        let mut ret = Vec::with_capacity(Self::IPV6_LEN);
        for (from, key) in self.from.iter().zip(self.to.iter()) {
            ret.push(from ^ key);
        }
        ret
    }
}

pub async fn calculate_ipv6_sub(ip_add_v6: Query<IPSubV6Query>) -> String {
    println!("--- Input(Sub) ---");
    println!("From: {}", ip_add_v6.from);
    println!("To: {}", ip_add_v6.to);
    let ip_v6 = IPAddV6::new(&ip_add_v6.from, &ip_add_v6.to);
    println!("model: {:?}", ip_v6);
    let ans = ip_v6.xor();
    // ansで0が連続している部分を省略して、::に置き換える
    let mut ret = String::new();
    let mut is_abbreviated = false;
    for (i, ip) in ans.iter().enumerate() {
        if *ip == 0 {
            if i == 0 {
                ret.push_str(":");
            }
            if !is_abbreviated {
                ret.push_str(":");
                is_abbreviated = true;
            }
        } else {
            ret.push_str(&format!("{:x}", ip));
            if i != IPAddV6::IPV6_LEN - 1 {
                ret.push(':');
            }
        }
    }
    println!("Ans: {}", ret);
    ret
}
