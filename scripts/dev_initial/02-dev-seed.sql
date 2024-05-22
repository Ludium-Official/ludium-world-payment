-- tb_ldm_usr mock 데이터
INSERT INTO public.tb_ldm_usr (id, nick, self_intro, phn_nmb) VALUES
    ('00000000-0000-0000-0000-000000000001', 'user1', 'Hello, I am user1.', '010-1111-2222'),
    ('00000000-0000-0000-0000-000000000002', 'user2', 'Hello, I am user2.', '010-2222-3333'),
    ('00000000-0000-0000-0000-000000000003', 'user3', 'Hello, I am user3.', '010-3333-4444'),
    ('00000000-0000-0000-0000-000000000004', 'user4', 'Hello, I am user4.', '010-4444-5555'),
    ('00000000-0000-0000-0000-000000000005', 'user5', 'Hello, I am user5.', '010-5555-6666');

-- tb_ldm_usr_rgh mock 데이터
INSERT INTO public.tb_ldm_usr_rgh (id, is_crt, is_prv, is_adm) VALUES
    ('00000000-0000-0000-0000-000000000001', true, false, false),
    ('00000000-0000-0000-0000-000000000002', false, true, false),
    ('00000000-0000-0000-0000-000000000003', false, false, true),
    ('00000000-0000-0000-0000-000000000004', true, true, false),
    ('00000000-0000-0000-0000-000000000005', false, true, true);