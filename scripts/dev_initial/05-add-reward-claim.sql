CREATE type reward_claim_status AS ENUM ('READY', 'PENDING_APPROVAL', 'TRANSACTION_APPROVED', 'TRANSACTION_FAILED', 'REJECTED');

CREATE TABLE public.reward_claim (
    id uuid NOT NULL,
    mission_id uuid NOT NULL,
    coin_network_id uuid NOT NULL,
    reward_claim_status reward_claim_status NOT NULL DEFAULT 'READY',
    amount numeric NOT NULL, -- 청구 금액
    user_id uuid NOT NULL, -- 받을 사용자 user id
    user_address varchar(100) NOT NULL, -- 받을 사용자 지갑 주소
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_claim_pk PRIMARY KEY (id)
);

CREATE TABLE public.reward_claim_detail (
    id uuid NOT NULL,
    reward_claim_id uuid NOT NULL,
    transaction_hash varchar(100) NOT NULL, -- 전송 트랜잭션 해시 값
    sended_user_id uuid NOT NULL, -- 보낸 사용자 user id
    sended_user_address varchar(100)NOT NULL, -- 보낸 사용자 지갑 주소
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_claim_detail_pk PRIMARY KEY (id),
    CONSTRAINT reward_claim_detail_fk_reward_claim FOREIGN KEY (reward_claim_id) REFERENCES public.reward_claim (id)
);