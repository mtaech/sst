export interface Resp<T> {
    code: number,
    msg: string | null,
    data: T | null
}

export interface Profile {
    name: String,
    sub_url: String,
    sub: Sub,
}

export interface Sub {
    remarks: string,
    status: string,
    servers: Server[] | null,
}

 export interface Server {
    name: string,
    server: String,
    server_port: number,
    password: string,
    method: string,
    mode: string,
    ss_url:string
}

