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

-- Insert into detailed_posting table
INSERT INTO public.detailed_posting (detail_id, posting_id, title, description, deadline, status, is_pinned, pin_order, reward_token, reward_amount)
VALUES
    ('33333333-0000-0000-0000-000000000001', '44444444-0000-0000-0000-000000000001', 'First Posting', 'Description for the first posting', '2024-12-31 23:59:59', 'CREATE', false, -1, '22222222-0000-0000-0000-000000000001', 100.00),
    ('33333333-0000-0000-0000-000000000002', '44444444-0000-0000-0000-000000000002', 'Second Posting', 'Description for the second posting', '2024-11-30 23:59:59', 'APPROVE', true, 1, '22222222-9c58-47f8-9a0f-2d0c8d3f807f', 200.00),
    ('33333333-0000-0000-0000-000000000003', '44444444-0000-0000-0000-000000000003', 'Third Posting', 'Description for the third posting', '2024-10-31 23:59:59', 'CLOSED', false, -1, '33333333-9c58-47f8-9a0f-2d0c8d3f807f', 300.00),
    ('33333333-0000-0000-0000-000000000004', '44444444-0000-0000-0000-000000000004', 'Fourth Posting', 'Description for the fourth posting', '2024-09-30 23:59:59', 'CREATE', true, 2, '22222222-0000-0000-0000-000000000001', 150.00),
    ('33333333-0000-0000-0000-000000000005', '44444444-0000-0000-0000-000000000005', 'Fifth Posting', 'Description for the fifth posting', '2024-08-31 23:59:59', 'APPROVE', false, -1, '22222222-9c58-47f8-9a0f-2d0c8d3f807f', 250.00);
