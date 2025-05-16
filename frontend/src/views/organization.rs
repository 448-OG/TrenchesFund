use dioxus::prelude::*;
use solana_payments::SolanaPayUrl;
use trenchesfund_common::{Outcome, Publisher, REST_ENDPOINT};
use wallet_adapter::web_sys;

use crate::{utils::copied_address, CopySvg, Loader, NotificationInfo, ReceiveSvg, GLOBAL_MESSAGE};

#[component]
pub fn OrganizationView(id: Vec<String>) -> Element {
    let mut publisher_info = use_signal(|| Option::<Publisher>::default());
    let mut show_receive_modal = use_signal(|| false);
    let mut mint_address = use_signal(|| String::default());

    // web_sys::console::log_1(&"EXEC 1".into());

    let id = id.first().cloned();

    let send_err_notify = move |msg: &str| {
        GLOBAL_MESSAGE
            .write()
            .push_back(NotificationInfo::error(msg));
        publisher_info.clone().write().take();
    };

    use_effect(move || {
        if let Some(id_valid) = id.as_ref().cloned() {
            spawn(async move {
                let endpoint =
                    String::new() + REST_ENDPOINT + "/publisher-info/" + id_valid.as_str();

                if let Ok(response) = reqwest::Client::new().post(&endpoint).send().await {
                    let body = if let Ok(text) = response.text().await {
                        text
                    } else {
                        send_err_notify("Unable to get the body text");
                        return;
                    };

                    if let Ok(data) = serde_json::from_str::<Outcome<Option<Publisher>>>(&body) {
                        match data {
                            Outcome::Success(success_data) => {
                                let success_data = success_data.unwrap();

                                *mint_address.write() = success_data.mint_address().clone();

                                publisher_info.write().replace(success_data);
                            }
                            Outcome::Failure(failure_data) => {
                                send_err_notify(&failure_data);
                            }
                        }
                    } else {
                        send_err_notify(
                            "Unable to fetch publisher! Check your connection and try again",
                        );
                        publisher_info.write().take();
                    }
                } else {
                    send_err_notify(
                        "Unable to fetch publisher! Check your connection and try again",
                    );
                    publisher_info.write().take();
                }
            });
        } else {
            web_sys::console::log_1(&"INVALID ID".into());
            send_err_notify("The route received an invalid id");
        }
    });

    rsx! {
        if let Some(publisher) = publisher_info.read().as_ref() {

                div { class: "flex flex-col justify-center items-center  w-full",
                    div { class: "flex flex-col justify-center items-center p-5 w-full",
                        div { class: "flex w-full items-center justify-center",
                            div { class: "w-[200px] rounded-xl mt-10 p-2 shadow-soft-dark",
                                    img {
                                        class: "w-full h-auto",
                                        src: publisher.icon.as_str(),
                                    }
                                }
                        }
                        div { class: "flex w-full items-center justify-center text-wrap mt-8",
                            h1 { class: "text-3xl text-wrap flex flex-col break-all", {publisher.name.as_str()} }
                        }
                        div { class: "flex flex-col w-[90%] mt-10 text-center text-wrap text-sm",
                            h1 { {publisher.description.as_str()} }
                        }
                    }
                    div { class: "flex flex-col w-full content-evenly mb-10",
                        div { class: "flex w-full p-5 flex-wrap gap-8 items-center justify-center",
                            div { class: "flex",
                                div { class: "flex w-[20px] mr-2",
                                    img {
                                        alt: "",
                                        src: "/images/codebase.svg",
                                        srcset: "",
                                    }
                                }
                                div { class: "flex",
                                    a {
                                        class: "text-blue-200",
                                        href: publisher.codebase.as_str(),
                                        "Codebase⇗"
                                    }
                                }
                            }
                            div { class: "flex",
                                div { class: "flex w-[20px] mr-2",
                                    img {
                                        alt: "",
                                        src: "/images/codebase.svg",
                                        srcset: "",
                                    }
                                }
                                div { class: "flex",
                                    a {
                                        class: "text-blue-200",
                                        href: publisher.website.as_str(),
                                        "Website⇗"
                                    }
                                }
                            }
                            div { class: "flex",
                                div{class:"mb-2 mt-5 rounded-full bg-rich-black hover:bg-true-blue cursor-pointer",
                                onclick:move|_| {

                                    spawn(async move {
                                        if let Err(error) = copied_address(mint_address.read().as_str()).await {
                                            GLOBAL_MESSAGE.write().push_back(NotificationInfo::error(format!("COPY ERROR: {:?}", error)));
                                        } else {
                                            GLOBAL_MESSAGE.write().push_back(NotificationInfo::new("Copied to clipboard"));
                                        }
                                    });
                                },
                                div { class:"flex justify-left items-center text-white py-0.5 px-1 rounded-full ",
                                    span {class:"flex p-2 w-[30px]", {CopySvg()} }
                                    span { "Mint Address" }
                                }
                            }
                            }


                        }
                    }
                    div { class: "flex w-full items-center justify-center gap-8",
                        div { class: "flex",
                            a {
                                class: "text-white bg-true-blue rounded-full py-2 px-4",
                                href: "#sponsor",
                                "Sponsor or Tip"
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-col w-full min-h-screen p-5 items-center justify-center gap-12",
                    id: "sponsor",
                    div { class: "flex flex-col items-center justify-center text-center",
                        div { class: "flex w-full",
                            span { class: "flex w-[30px] mr-1",
                                img { alt: "", src: "/images/icon.svg", srcset: "" }
                            }
                            span {
                                h1 { class: "flex text-3xl", "Tip, Airdrop or Sponsor" }
                            }
                        }
                        div { class: "flex text-center text-sm mt-5",
                            "You can tip, sponsor or airdrop a publisher to keep their lights on."
                        }
                    }
                    div { class: "flex w-full items-center justify-center gap-8",
                        div { class: "flex",
                            button {
                                onclick:move |_|{
                                    show_receive_modal.set(true);
                                },
                                class: "text-white bg-true-blue rounded-full py-2 px-4",
                                "Solana Pay"
                            }
                        }

                        div { class: "flex",
                            a {
                                class: "text-white bg-true-blue rounded-full py-2 px-4",
                                href: "#merch",
                                "Buy Merch"
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-col w-full min-h-screen p-5 items-center justify-center",
                    id: "merch",
                    div { class: "mb-20 flex flex-col items-center justify-center text-center",
                        div { class: "flex w-full",
                            span { class: "flex w-[30px] mr-1",
                                img { alt: "", src: "/images/icon.svg", srcset: "" }
                            }
                            span {
                                h1 { class: "flex text-3xl", "Buy Solana Merch to Support the Project" }
                            }
                        }
                    }

                     div { class: "flex text-center p-5 gap-8 flex-wrap items-center justify-center",
                          for merch in publisher.merch.as_slice() {
                            div { class: "flex w-[500px] rounded-2xl p-1 bg-true-blue justify-center items-center",
                                div { class: "flex w-full h-auto",
                                    img {
                                        alt: "",
                                        class: "rounded-2xl",
                                            src: merch.image.as_str(),
                                        }
                                }
                                div { class: "flex flex-col items-center justify-center w-full h-full",
                                    div { class: "flex flex-col items-center justify-center  text-center p-2",
                                        span{class:"text-wrap break-all text-2xl text-bold font-smooch mb-1", {merch.name.as_str()}}
                                    }
                                    div { class: "flex flex-col items-center justify-center  text-center text-lg",
                                        span{class:"text-sm mb-5 text-wrap p-2", {merch.description.as_str()}}
                                    }
                                    div { class: "flex flex-col items-center justify-center  text-center p-2 text-lg",
                                        span {class:"flex text-sm", "Supplier: "} span{class:"text-lg mb-5 font-smooch", {merch.supplier.as_str()}}
                                    }
                                    div { class: "flex items-center justify-center",
                                        div { class: "flex",
                                            a {
                                                class: "bg-white text-true-blue rounded-full py-1 px-8",
                                                href: "#",
                                                "Buy"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                ReceiveTokens{show_receive_modal, publisher: publisher.clone()}

        } else {
                {Loader()}

        }
    }
}

#[component]
pub fn ReceiveTokens(show_receive_modal: Signal<bool>, publisher: Publisher) -> Element {
    let solana_pay_url = SolanaPayUrl::default()
        .add_recipient(&publisher.address())
        .unwrap_or_default()
        .add_label(&publisher.name)
        .unwrap_or_default()
        .add_message("Thanks for buying coffee and keeping the lights on.")
        .unwrap_or_default()
        .to_url();

    let qrcode = if let Ok(qr) = crate::address_qrcode(&solana_pay_url) {
        qr
    } else {
        rsx! {
            div { class:"text-black dark:text-white", }
        }
    };

    let address_inner = publisher.address().clone();

    if *show_receive_modal.read() {
        rsx! {
            div {
                class: "fixed overflow-y-hidden min-h-screen z-10 flex flex-col w-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white",
                div { class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl",
                    div { class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick:move|_|{show_receive_modal.set(false)},
                            class: "wallet-adapter-modal-button-close w-[30px] items-center justify-center",
                            "data-dioxus-id": "65",
                            svg {
                                fill: "none",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "m15 9.00004-6 5.99996m6 0-6-5.99996m3 11.99996c4.9706 0 9-4.0294 9-9 0-4.97056-4.0294-9-9-9-4.97056 0-9 4.02944-9 9 0 4.9706 4.02944 9 9 9z",
                                    stroke: "#a6c1ee",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                }
                            }
                        }
                    }
                    div { class: "overflow-y-scroll w-full mb-5 items-center justify-center flex flex-col",
                        div { class: "flex w-full items-center justify-center text-2xl", span{class:"w-[30px] font-smooch mr-2",{ReceiveSvg()}} "Tip, Sponsor, Airdrop" }
                        div{class:"mb-2 mt-5 rounded-full bg-true-blue hover:bg-cobalt-blue cursor-pointer",
                            onclick:move|_| {
                                let address_inner = address_inner.clone();

                                spawn(async move {
                                    if let Err(error) = copied_address(&address_inner).await {
                                        GLOBAL_MESSAGE.write().push_back(NotificationInfo::error(format!("COPY ERROR: {:?}", error)));
                                    } else {
                                        GLOBAL_MESSAGE.write().push_back(NotificationInfo::new("Copied to clipboard"));
                                    }
                                });
                            },
                            div { class:"flex justify-left items-center px-2 py-1 text-white rounded-full ",
                                span {class:"flex p-2 w-[30px]", {CopySvg()} }
                                span { {publisher.short_address()} }
                            }
                        }
                        div{class:"w-[200px] rounded-xl flex flex-col mt-5 mb-5 bg-white",
                            div{class:"w-full flex items-center text-center justify-center",
                                {qrcode}
                            }
                        }

                            div {class:"w-full flex items-center text-center justify-center font-smooch text-xl",
                                "Solana Pay QR Code"
                            }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
