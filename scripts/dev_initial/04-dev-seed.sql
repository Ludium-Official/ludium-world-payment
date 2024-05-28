-- Insert into tb_ldm_usr table
INSERT INTO public.tb_ldm_usr (id, nick, self_intro, phn_nmb) VALUES
    ('00000000-0000-0000-0000-000000000001', 'user1', 'Hello, I am user1.', '010-1111-2222'),
    ('00000000-0000-0000-0000-000000000002', 'user2', 'Hello, I am user2.', '010-2222-3333'),
    ('00000000-0000-0000-0000-000000000003', 'user3', 'Hello, I am user3.', '010-3333-4444'),
    ('00000000-0000-0000-0000-000000000004', 'user4', 'Hello, I am user4.', '010-4444-5555'),
    ('00000000-0000-0000-0000-000000000005', 'user5', 'Hello, I am user5.', '010-5555-6666');

-- Insert into tb_ldm_usr_rgh table
INSERT INTO public.tb_ldm_usr_rgh (id, is_crt, is_prv, is_adm) VALUES
    ('00000000-0000-0000-0000-000000000001', true, false, false),
    ('00000000-0000-0000-0000-000000000002', false, true, false),
    ('00000000-0000-0000-0000-000000000003', false, false, true),
    ('00000000-0000-0000-0000-000000000004', true, true, false),
    ('00000000-0000-0000-0000-000000000005', false, true, true);

-- Insert into network table
INSERT INTO public.network (id, name, code) VALUES 
('86b82d0f-80d5-4406-ae5a-e387db79ca32', 'NEAR Protocol', 'NEAR');

-- Insert into coin table
INSERT INTO public.coin (id, name, symbol, coin_type) VALUES 
('5cb2dca4-b693-49b5-bd20-00ddce72d54b', 'USD Tether', 'USDT', 'FT'),
('a3d281dd-4f85-4e5e-b639-b5bf1d8ee853', 'USD Coin', 'USDC', 'FT');

-- Insert into coin_network table
INSERT INTO public.coin_network (id, coin_id, network_id, contract_address) VALUES 
('1859ebb9-d031-473a-8241-b0b6832c2652', '5cb2dca4-b693-49b5-bd20-00ddce72d54b', '86b82d0f-80d5-4406-ae5a-e387db79ca32', 'tt_local.testnet'),
('3e6d84d8-9c58-47f8-9a0f-2d0c8d3f807f', 'a3d281dd-4f85-4e5e-b639-b5bf1d8ee853', '86b82d0f-80d5-4406-ae5a-e387db79ca32', 'usdc.contract.near');

-- Insert into reward_claim table
INSERT INTO public.reward_claim (id, mission_id, coin_network_id, reward_claim_status, amount, user_id, user_address) VALUES 
('1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d', '5f4d3c2b-1a0e-9f8d-7c6b-5a4d3c2e1f0a', '1859ebb9-d031-473a-8241-b0b6832c2652', 'READY', 100.00, 'd7f6e5c4-b3a2-1f0e-9d8c-7b6a5d4e3f2c', 'user_wallet_address_1'),
('2b3c4d5e-6f7a-8b9c-0d1e-2f3a4b5c6d7e', '4d3c2b1a-0e9f-8d7c-6b5a-4d3c2e1f0a5d', '3e6d84d8-9c58-47f8-9a0f-2d0c8d3f807f', 'TRANSACTION_APPROVED', 200.00, 'e5c4d3b2-a1f0-9e8d-7c6b-5a4d3c2f1e0d', 'user_wallet_address_2'),
('3c4d5e6a-7a8b-9c0d-1e2a-3a41111d0000', '2b1a0e9f-8d7c-6b5a-4d3c-abcd2e1f0a5d', '1859ebb9-d031-473a-8241-b0b6832c2652', 'READY', 300.00, '4d3c2b1a-0e9f-8d7c-6b5a-4d3c2e1f0a5d', 'user_wallet_address_3'),
('4d5e6f7a-8b9c-0d1e-2f3a-4a1234c6d000', '3c2b1a0e-9f8d-7c6b-5a4d-abcd3c2e1f0a', '3e6d84d8-9c58-47f8-9a0f-2d0c8d3f807f', 'READY', 400.00, '5e6f7a8b-9c0d-1e2f-3a4b-abcd5c6d7e8f', 'user_wallet_address_4');

-- Insert into reward_claim_detail table
INSERT INTO public.reward_claim_detail (id, reward_claim_id, transaction_hash, sended_user_id, sended_user_address) VALUES 
('4d5e6f7a-8b9c-0d1e-2f3a-4b5c6d7e8f9a', '2b3c4d5e-6f7a-8b9c-0d1e-2f3a4b5c6d7e', 'tx_hash_2', 'b2a1f0e9-8d7c-6b5a-4d3c-2f1e0d2b3c4d', 'sender_wallet_address_2');
