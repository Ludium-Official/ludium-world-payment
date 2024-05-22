---- Base app schema

CREATE TABLE public.tb_ldm_usr (
    id uuid NOT NULL, -- 아이디
    nick varchar(30) NOT NULL DEFAULT ''::character varying, -- 닉네임
    self_intro varchar(100) NOT NULL DEFAULT ''::character varying, -- 자기소개
    phn_nmb varchar NOT NULL, -- 전화번호
    CONSTRAINT tb_ldm_usr_pk PRIMARY KEY (id)
);

CREATE TABLE public.tb_ldm_usr_rgh (
    id uuid NOT NULL, -- 아이디
    is_crt bool NOT NULL DEFAULT false, -- 컨트리뷰터 여부
    is_prv bool NOT NULL DEFAULT false, -- 프로바이더 여부
    is_adm bool NOT NULL DEFAULT false, -- 관리자 여부
    CONSTRAINT usr_rgh_pk PRIMARY KEY (id),
    CONSTRAINT usr_rgh_fk FOREIGN KEY (id) REFERENCES public.tb_ldm_usr(id)
);