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
