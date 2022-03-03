// Copyright (C) 2020 Christian Amsüss
//
// This file is subject to the terms and conditions of the GNU Lesser
// General Public License v2.1. See the file LICENSE in the top level
// directory for more details.
#![no_main]
#![no_std]

use riot_build as _;
use riot_rs_rt as _;

use riot_wrappers::{gcoap, gnrc, stdio::println, thread, ztimer};

use coap_handler_implementations::{HandlerBuilder, ReportingHandlerBuilder};

#[no_mangle]
fn riot_main() {
    extern "C" {
        fn do_vfs_init();
    }

    unsafe { do_vfs_init() };

    let handler = coap_message_demos::full_application_tree(None)
        .below(&["ps"], riot_coap_handler_demos::ps::ps_tree())
        .below(&["vfs"], riot_coap_handler_demos::vfs::vfs("/const"))
        .with_wkc();
    let mut handler = riot_wrappers::coap_handler::GcoapHandler(handler);

    let mut listener = gcoap::SingleHandlerListener::new_catch_all(&mut handler);

    gcoap::scope(|greg| {
        greg.register(&mut listener);

        println!(
            "CoAP server ready; waiting for interfaces to settle before reporting addresses..."
        );

        let sectimer = ztimer::ZTimer::sec();
        sectimer.sleep_ticks(5);

        for netif in gnrc::Netif::all() {
            println!(
                "Active interface from PID {:?} ({:?})",
                netif.pid(),
                netif.pid().get_name().unwrap_or("unnamed")
            );
            match netif.ipv6_addrs() {
                Ok(addrs) => {
                    for a in addrs.addresses() {
                        println!("    Address {:?}", a);
                    }
                }
                _ => {
                    println!("    Does not support IPv6.");
                }
            }
        }

        // Sending main thread to sleep; can't return or the Gcoap handler would need to be
        // deregistered (which it can't).
        loop {
            thread::sleep();
        }
    })
}
