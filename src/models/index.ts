export type Resp<T> = {
    code: number,
    msg: string | null,
    data: T | null
}

export type Profile ={
    name: String,
    sub_url: String,
    sub: Sub,
}

export type Sub ={
    remarks: string,
    status: string,
    servers: Server[] | null,
}

 export type Server ={
    name: string,
    server: String,
    server_port: number,
    password: string,
    method: string,
    mode: string,
    ss_url:string
}

