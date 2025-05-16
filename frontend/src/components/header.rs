use dioxus::prelude::*;

use crate::{
    utils::copied_address, ChangeWalletSvg, CloseSvg, CopySvg, DisconnectSvg, GradientWalletIcon,
    Loader, NotificationInfo, Route, WalletSvg, ACTIVE_CONNECTION, GLOBAL_MESSAGE, LOGO,
    WALLET_ADAPTER,
};

#[component]
pub fn Header() -> Element {
    let show_modal = use_signal(|| false);
    let show_connecting = use_signal(|| false);

    let shortened_address = String::default();

    rsx! {
        div { class:"flex flex-col w-full gap-4 justify-between items-center",
            nav {class:"flex w-full justify-around items-center p-1 dark:shadow-lg shadow-sm border-b-[1px] dark:border-true-blue",
                div{class:"p-1 flex items-center", span{class:"w-[25px]",img{src:LOGO, alt:"LOGO"}}, span {class:"text-3xl font-smooch", "Trenches" } }
                div{ class:"flex items-center justify-around w-[80%] mx-2",
                    {NavItem(Route::Home, "Home")}
                    {NavItem(Route::Explore, "Explore")}
                }
                NavWalletItem{show_modal, show_connecting, shortened_address}
            }
        }


        ConnectWalletModalModal { show_modal, show_connecting }

        Outlet::<Route> {}
    }
}

#[component]
pub fn ConnectWalletModalModal(show_modal: Signal<bool>, show_connecting: Signal<bool>) -> Element {
    if *show_modal.read() {
        rsx! {
            div{class:"flex flex-col w-full h-full bg-[#1a1a1a88] absolute items-center justify-center z-50",

                div { class: "flex relative w-full max-w-[90%] lg:max-w-[40%] md:max-w-[55%] max-h-full",
                    if !WALLET_ADAPTER.read().wallets().is_empty() {
                        div { class: "relative bg-white rounded-lg shadow-lg dark:bg-rich-black items-center justify-between flex flex-col w-full h-full min-h-[40vh]",
                            div { class: "flex items-center justify-center p-4 md:p-5 rounded-t w-full dark:border-gray-600 border-gray-200",
                                div {
                                    class:"flex w-5/6 items-center justify-center",
                                    h3 { class: "text-2xl flex items-center justify-center font-semibold text-blue-yonder dark:text-white",
                                        span{class:"w-[30px] mr-2 flex", {GradientWalletIcon()}} "Connect A Wallet"
                                    }
                                }
                                div { class:"flex w-1/6",
                                    button {
                                        onclick:move|_| {show_modal.set(false);},
                                        class: "text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline- justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white",
                                        "data-modal-hide": "default-modal",
                                        r#type: "button",
                                        {CloseSvg()}
                                        span { class: "sr-only", "Close modal" }
                                    }
                                }
                            }
                            ul { class: "flex space-y-4 mb-5 w-full justify-center flex-col items-center h-full",
                                for wallet in WALLET_ADAPTER.read().wallets().clone() {
                                    li {
                                        onclick:move|_|{
                                            let wallet = wallet.clone();

                                            spawn(async move {
                                                show_modal.set(false);
                                                show_connecting.set(true);

                                                if let Err(error) = WALLET_ADAPTER.write().connect(wallet).await {
                                                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(error));
                                                }

                                                show_connecting.set(false);
                                            });
                                        },
                                        class: "flex justify-center cursor-pointer w-full text-lg hover:bg-true-blue  text-true-blue hover:text-white dark:text-white px-4 py-2",
                                        div{class:"max-w-[80%] flex justify-between w-full",
                                            div {class:"flex items-center",
                                                if let Some(icon) = wallet.icon() {
                                                    img {class:"flex w-[25px] mr-2 items-center", src:icon.to_string(), alt:wallet.name()}
                                                }else {
                                                    span {class:"flex w-[25px] mr-2 items-center", {WalletSvg()}}
                                                }
                                                span {class:"flex", {wallet.name()}  }
                                            }
                                            span {class:"flex", "Detected"  }
                                        }
                                    }
                                }
                            }
                        }
                    }else {
                        div { class: "relative bg-white rounded-lg shadow-lg dark:bg-rich-black items-center justify-start p-2 flex flex-col w-full h-full min-h-[40vh]",
                                div { class:"flex w-full mr-5",
                                    button {
                                        onclick:move|_|{
                                            show_modal.set(false);
                                        },
                                        class: "text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline- justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white",
                                        "data-modal-hide": "default-modal",
                                        r#type: "button",
                                        {CloseSvg()}
                                        span { class: "sr-only", "Close modal" }
                                    }
                                }
                                div{class:"flex text-2xl w-full p-5 flex-col items-center justify-around h-full",
                                    div{class:"flex w-full items-center justify-center",
                                        span{class:"flex w-[50px] mr-5 items-center", {GradientWalletIcon()}}
                                        span{"No Solana Wallets Detected"},
                                    }
                                    div {class:"flex text-lg", "Install a Solana Wallet Installed on your browser!"}
                                }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}

fn NavItem(route: fn() -> Route, text: &str) -> Element {
    rsx! {
        Link {class:"w-[10%] font-smooch text-xl hover:bg-transparent dark:text-blue-yonder dark:hover:text-white text-true-blue hover:text-black rounded-lg text-center p-1", to: route(), {text}}
    }
}

#[component]
fn NavWalletItem(
    show_modal: Signal<bool>,
    show_connecting: Signal<bool>,
    shortened_address: String,
) -> Element {
    let compute_wallet = || {
        if let Ok(connected_account) = ACTIVE_CONNECTION.read().connected_account() {
            let shortened_address = connected_account.shorten_address().unwrap();

            rsx! { ActiveAccountDropDown{show_modal, shortened_address} }
        } else {
            rsx! {
                div {class:"flex w-full items-center justify-center",
                button {class:"text-sm md:text-lg font-smooch min-w-[60px]",
                    onclick:move|_|{show_modal.set(true);},
                        "Select Wallet"
                    }
                }
            }
        }
    };

    rsx! {
        div { class:"w-[25%] flex ml-2 text-white py-1 px-4 appearance-none items-center justify-center cursor-pointer",
            if *show_connecting.read() {
                div {class:"py-1 px-4 flex items-center justify-center hover:bg-true-yonder bg-true-blue rounded-full",
                    span{class:"flex w-[20px] mr-5", {WalletSvg()}}
                    span{class:"flex mr-5", {Loader()}}
                }
            } else {
                div{class:"flex hover:bg-true-yonder bg-true-blue text-white rounded-full py-1 px-4 appearance-none text-center cursor-pointer",
                    {compute_wallet()}
                }
            }
        }
    }
}

#[component]
pub fn ActiveAccountDropDown(show_modal: Signal<bool>, shortened_address: String) -> Element {
    let mut show_dropdown = use_signal(|| false);

    let disconnect_callback = move || {
        spawn(async move {
            WALLET_ADAPTER.write().disconnect().await;
        });
    };

    let clone_address = shortened_address.clone();

    let copy_callback = move || {
        let inner_address = clone_address.clone();
        spawn(async move {
            if let Err(error) = copied_address(&inner_address).await {
                GLOBAL_MESSAGE
                    .write()
                    .push_back(NotificationInfo::new(error));
            } else {
                GLOBAL_MESSAGE
                    .write()
                    .push_back(NotificationInfo::new("Copied to clipboard"));
            }
        });
    };

    let change_wallet_callback = move || {
        show_modal.set(true);
    };

    let connected_wallet = ACTIVE_CONNECTION.read().connected_wallet().unwrap().clone();

    rsx! {
        div {
            class:"relative inline-block rounded-full",
            div {
                onclick:move|_| {
                    if *show_dropdown.read() {
                        show_dropdown.set(false);
                    }else {
                        show_dropdown.set(true);
                    }
                },
                class:"flex w-full text-center items-center justify-center",
                span{class:"flex w-[20px] mr-2",
                    if let Some(icon) = connected_wallet.icon() {
                        img{class:"rounded-lg", src:icon.to_string()}
                    }else {
                        {WalletSvg()}
                    }
                }
                {shortened_address}
            }

            if *show_dropdown.read() {
                ul {class:"w-full min-w-[130px] text-white flex flex-col absolute z-1 text-md mt-2 bg-true-blue rounded-lg shadow-xl list-none",
                    {DropdownItem("Copy Address", CopySvg(), show_dropdown, copy_callback)}
                    {DropdownItem("Change Wallet", ChangeWalletSvg(), show_dropdown, change_wallet_callback)}
                    {DropdownItem("Disconnect", DisconnectSvg(), show_dropdown, disconnect_callback)}
                }
            }
        }
    }
}

fn DropdownItem<F>(
    value: &str,
    icon: Element,
    mut show_dropdown: Signal<bool>,
    mut callback: F,
) -> Element
where
    F: FnMut() + 'static,
{
    rsx! {
        li{class:"flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer",
            onclick:move|_| {
                show_dropdown.set(false);
                callback();
            },
            div { class:"flex text-sm justify-left items-center ",
                span {class:"p-2 w-[30px]", {icon} }
                span { {value} }
            }
        }
    }
}
