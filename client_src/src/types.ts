export interface Ticket {
    id?: number,
    subject: string,
    state_title?: string,
    created: string,
    changed: string
}

export interface LoginResponse {
    token: string;
    success: boolean
}