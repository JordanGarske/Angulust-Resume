export interface ClientReview{
    id: number;
    review_id: number;
    first_name: string;
    last_name: string;
    client_password: string;
    email: string;
    admin_privilege: boolean;
    phone_number: string | null; // Adjusted based on SQL definition
    profession: string;
    company: string;
    elucidation: string;
}
export interface Client  {
    id: number;
    first_name: string;
    last_name: string;
    client_password: string;
    email: string;
    admin_privilege: boolean;
    phone_number: string | null; // Adjusted based on SQL definition
    profession: string;
    company: string;
  }

  