CREATE TABLE IF NOT EXISTS messages (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  github_username STRING NOT NULL,
  sms_location JSONB,
  function_used STRING NOT NULL,
  created_on TIMESTAMPTZ DEFAULT now()
);