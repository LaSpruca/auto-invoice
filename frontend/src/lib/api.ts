import { userToken } from "./stores";
import axios from "axios";
import { get } from "svelte/store";

export async function getPrivileged(route: string, params?: Record<string, any>) {    
    let token = get(userToken);

    if (!token) {
        throw new Error("Not logged in");
    }
    
    return axios.get(`http://localhost:8000${route}${(params ? "?" + new URLSearchParams(params).toString() : "")}`, {
        headers: {
            Authorization: `Bearer ${token}`
        }
    })
};


