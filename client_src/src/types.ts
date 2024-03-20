export interface Ticket {
    id?: number,
    subject: string,
    content: string,
    state_title?: string,
    created: string,
    changed: string,
    client_id: number;
}

export interface Client {
    id: number, 
    name: string,
    email: string
}

export interface PropertyInfo {
    id: number, 
    owner_id: number | null,
    data_type: string
    data_value: string | null
}

export interface LoginResponse {
    token_value: string;
    user_id: number;
    valid_to: string
}