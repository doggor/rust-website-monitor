import * as axios from "axios";

export async function listSites() {
    const response = await axios.get(`/sites`);
    return response.data;
}

export async function createSite(domain) {
    const response = await axios.post(`/sites`, {
        domain,
    });
    return response.data;
}

export async function updateSite(id, domain, active) {
    const response = await axios.put(`/sites/${id}`, {
        domain,
        active: typeof active === "string" ? (active === "true" ? true : false) : active,
    });
    return response.data;
}

export async function removeSite(id) {
    const response = await axios.delete(`/sites/${id}`);
    return response.data;
}