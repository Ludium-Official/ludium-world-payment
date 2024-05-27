CREATE TYPE coin_type AS ENUM ('NATIVE', 'FT', 'NFT');

CREATE TABLE public.coin (
    id uuid NOT NULL,
    name varchar(50) NOT NULL,
    symbol varchar(10) NOT NULL,
    coin_type coin_type NOT NULL,
    CONSTRAINT coin_pk PRIMARY KEY (id)
);

CREATE TABLE public.network (
    id uuid NOT NULL,
    name varchar(50) NOT NULL,
    code varchar(20) NOT NULL,
    CONSTRAINT network_pk PRIMARY KEY (id)
);

CREATE TABLE public.coin_network (
    id uuid NOT NULL,
    coin_id uuid NOT NULL,
    network_id uuid NOT NULL,
    contract_address varchar(100),
    CONSTRAINT coin_network_pk PRIMARY KEY (id),
    CONSTRAINT coin_network_fk_coin FOREIGN KEY (coin_id) REFERENCES public.coin (id),
    CONSTRAINT coin_network_fk_network FOREIGN KEY (network_id) REFERENCES public.network (id)
);
