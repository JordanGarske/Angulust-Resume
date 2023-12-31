export interface User {
  id: number;
  resume_reference_id:number;
  first_name: string;
  last_name: string;
  client_password: string;
  email: string;
  admin_privilege: boolean;
  phone_number: string | null; // Adjusted based on SQL definition
  profession: string;
  company: string;
}

export interface UserSignUp {
  client_password: string;
  email: string;
  admin_privilege: boolean;
  first_name: string;
  last_name: string;
  phone_number: string | null; // Adjusted based on SQL definition
  profession: string;
  company: string;
}

export interface UserLogin {
  email: string;
  client_password: string;
}

