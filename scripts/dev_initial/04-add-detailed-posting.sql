
CREATE TABLE public.detailed_posting (
    detail_id uuid NOT NULL,
    posting_id uuid NOT NULL,
    title varchar(255),
    description text,
    deadline timestamp,
    status varchar(50) NOT NULL, -- CREATE, APPROVE, CLOSED 
    is_pinned boolean NOT NULL DEFAULT false,
    pin_order integer NOT NULL DEFAULT -1,
    reward_token uuid, -- 보상 토큰 uuid (coin_network_id)
    reward_amount numeric, -- 보상 금액
    create_at timestamp NOT NULL DEFAULT now(),
    update_at timestamp NOT NULL DEFAULT now(),
    CONSTRAINT detailed_posting_pk PRIMARY KEY (detail_id)
);
