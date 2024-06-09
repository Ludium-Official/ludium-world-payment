-- Insert into tb_ldm_usr table
INSERT INTO public.tb_ldm_usr (id, nick, self_intro, phn_nmb) VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin', 'Hello, I am admin.', '010-1111-2222'),
    ('00000000-0000-0000-0000-000000000002', 'provider', 'Hello, I am provider.', '010-2222-3333'),
    ('00000000-0000-0000-0000-000000000003', 'contributor', 'Hello, I am contributor', '010-3333-4444'),
    ('00000000-0000-0000-0000-000000000004', 'user4', 'Hello, I am user4.', '010-4444-5555'),
    ('00000000-0000-0000-0000-000000000005', 'user5', 'Hello, I am user5.', '010-5555-6666');

-- Insert into tb_ldm_usr_rgh table
INSERT INTO public.tb_ldm_usr_rgh (id, is_crt, is_prv, is_adm) VALUES
    ('00000000-0000-0000-0000-000000000001', true, true, true),
    ('00000000-0000-0000-0000-000000000002', false, true, false),
    ('00000000-0000-0000-0000-000000000003', true, false, false),
    ('00000000-0000-0000-0000-000000000004', true, true, true),
    ('00000000-0000-0000-0000-000000000005', false, true, true);

-- Insert into mission table
INSERT INTO public.mission (mission_id, curriculum_id, title, description, usr_id, mission_submit_form) VALUES
    ('10000000-0000-0000-0000-000000000001', '20000000-0000-0000-0000-000000000001', 'Mission 1', 'Description for mission 1', '00000000-0000-0000-0000-000000000001', 'Form for mission 1'),
    ('10000000-0000-0000-0000-000000000002', '20000000-0000-0000-0000-000000000002', 'Mission 2', 'Description for mission 2', '00000000-0000-0000-0000-000000000002', 'Form for mission 2'),
    ('10000000-0000-0000-0000-000000000003', '20000000-0000-0000-0000-000000000003', 'Mission 3', 'Description for mission 3', '00000000-0000-0000-0000-000000000003', 'Form for mission 3'),
    ('10000000-0000-0000-0000-000000000004', '20000000-0000-0000-0000-000000000004', 'Mission 4', 'Description for mission 4', '00000000-0000-0000-0000-000000000004', 'Form for mission 4'),
    ('10000000-0000-0000-0000-000000000005', '20000000-0000-0000-0000-000000000005', 'Mission 5', 'Description for mission 5', '00000000-0000-0000-0000-000000000005', 'Form for mission 5');

-- Insert into mission_submit table
INSERT INTO public.mission_submit (mission_id, usr_id, description, status) VALUES
    ('10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000002', 'Submission for mission 1 by provider', 'SUBMIT'),
    ('10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'Submission for mission 2 by provider', 'APPROVE'),
    ('10000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000002', 'Submission for mission 3 by provider', 'APPROVE');

-- Insert into network table
INSERT INTO public.network (id, name, code) VALUES 
('86b82d0f-80d5-4406-ae5a-e387db79ca32', 'NEAR Protocol', 'NEAR');

-- Insert into coin table
INSERT INTO public.coin (id, name, symbol, coin_type, decimals) VALUES 
('11111111-0000-0000-0000-000000000001', 'USD Tether', 'USDT', 'FT', 6),
('11111111-0000-0000-0000-000000000002', 'USD Coin', 'USDC', 'FT', 6),
('11111111-0000-0000-0000-000000000003', 'NEAR', 'NEAR', 'NATIVE', 24);

-- Insert into coin_network table
INSERT INTO public.coin_network (id, coin_id, network_id, contract_address) VALUES 
('22222222-0000-0000-0000-000000000001', '11111111-0000-0000-0000-000000000001', '86b82d0f-80d5-4406-ae5a-e387db79ca32', 'tt_local.testnet'),
('22222222-9c58-47f8-9a0f-2d0c8d3f807f', '11111111-0000-0000-0000-000000000002', '86b82d0f-80d5-4406-ae5a-e387db79ca32', 'usdt.fakes.testnet'),
('33333333-9c58-47f8-9a0f-2d0c8d3f807f', '11111111-0000-0000-0000-000000000003', '86b82d0f-80d5-4406-ae5a-e387db79ca32', NULL);
