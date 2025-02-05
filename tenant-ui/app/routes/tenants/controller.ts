import useAPIClient from "@lib/api/APIClient";
import { Tenant } from "./model";

class TenantController {
  private apiClient = useAPIClient();

  async createTenant(data: Tenant) {
    try {
      const response = await this.apiClient.post("/tenants", data);
      return response;
    } catch (error) {
      throw new Error(`Failed to create tenant: ${error.message}`);
    }
  }

  async deleteTenant(tenantId: string) {
    try {
      const response = await this.apiClient.delete(`/tenants/${tenantId}`);
      return response;
    } catch (error) {
      throw new Error(`Failed to delete tenant: ${error.message}`);
    }
  }

  async getTenant(tenantId: string) {
    try {
      const response = await this.apiClient.get(
        ["tenant", tenantId],
        `/tenants/${tenantId}`
      );
      return response.data as Tenant;
    } catch (error) {
      throw new Error(`Failed to get tenant: ${error.message}`);
    }
  }

  async getTenants() {
    try {
      const response = await this.apiClient.get(["tenants"], "/tenants");
      return response.data as Tenant[];
    } catch (error) {
      throw new Error(`Failed to get tenants: ${error.message}`);
    }
  }

  async updateTenant(tenantId: string, data: Tenant) {
    try {
      const response = await this.apiClient.put(`/tenants/${tenantId}`, data);
      return response;
    } catch (error) {
      throw new Error(`Failed to update tenant: ${error.message}`);
    }
  }
}

export default TenantController;
