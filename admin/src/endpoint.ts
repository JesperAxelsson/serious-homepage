import axios, { Axios } from "axios";

export function createAxios(): Axios {
    return axios.create({ baseURL: 'http://localhost:3030' });
}

