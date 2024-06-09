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

CREATE TABLE public.mission (
	mission_id uuid NOT NULL,
	curriculum_id uuid NOT NULL,
	title varchar(255) NOT NULL,
	description text NOT NULL,
	create_at timestamp NOT NULL DEFAULT now(),
	usr_id uuid NOT NULL,
	mission_submit_form text NOT NULL,
	CONSTRAINT mission_pkey PRIMARY KEY (mission_id),
	CONSTRAINT mission_tb_ldm_usr_fk FOREIGN KEY (usr_id) REFERENCES public.tb_ldm_usr(id)
);

CREATE TABLE public.mission_submit (
	mission_id uuid NOT NULL,
	usr_id uuid NOT NULL,
	description text NOT NULL,
	status varchar(50) NOT NULL,
	create_at timestamp NOT NULL DEFAULT now(),
	CONSTRAINT mission_submit_pkey PRIMARY KEY (mission_id, usr_id),
	CONSTRAINT mission_submit_mission_id_fkey FOREIGN KEY (mission_id) REFERENCES public.mission(mission_id),
	CONSTRAINT mission_submit_usr_id_fkey FOREIGN KEY (usr_id) REFERENCES public.tb_ldm_usr(id)
);