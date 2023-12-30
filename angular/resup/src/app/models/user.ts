export interface User {
    client_id: number;
    first_name: string;
    last_name: string;
    client_password: string;
    email: string;
    admin_privilege: boolean;
  }
export interface User {
    first_name: string;
    last_name: string;
    email: string;
    personal_review_id: String,
    admin_privilege: boolean,
    phone_number: string
    profession: String,
    company: String,
  }
export interface UserSignUp {
  client_password: String,
  email: String,
  admin_privilege: boolean,
  first_name: String,
  last_name: String,
  phone_number: String|undefined,
  profession: String,
  company: String,

  }
export interface UserLogin {
  email: string;
  client_password: string;

  }