import axios from "axios";
import { user } from "./stores";

let userToken = null;

export function getPrivileged(route: string, params?: Record<string, any>) {
    const headers: Record<string, string> = {};

    if (userToken) {
        headers["Authorization"] = 'Bearer ' + userToken;
    }

    console.log(headers)

    return axios.get('localhost:8000' + route + (params ? "?" + new URLSearchParams(params).toString() : ""), headers)
};

user.subscribe(async (user) => {
    if (user) {
        userToken = await user.getIdToken();
    } else {
        userToken = null;
    }
}) ;