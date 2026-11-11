INSERT INTO users (
    id,
    email,
    password_hash
) VALUES (
    '00000000-0000-0000-0000-000000000000',
    'system',
    -- password: secret
    '$argon2id$v=19$m=19456,t=2,p=1$c2FsdHltY3NhbHRzdGVy$AEBiLPbgT23PDqzPWAzKakRLOGlb+DaHevR0mzzMA/8'
);
