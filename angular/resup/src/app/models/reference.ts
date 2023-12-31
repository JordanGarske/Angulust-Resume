export interface ClientReference {
    id: number;
    email: string;
    first_name: string;
    last_name: string;
    phone_number?: string | null; // Use ? for optional fields
    profession: string;
    company: string;
    elucidation: string;
  }
  export interface ResumeReference {
    client_id: number;
    elucidation: string;
  }
  