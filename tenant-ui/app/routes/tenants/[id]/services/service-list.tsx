import React, { Suspense, use, useState } from "react";
import { UseQueryResult } from "@tanstack/react-query";
import useAPIClient, { GetResult } from "@lib/api/APIClient";
import { Box } from "@mui/material";

interface Service {
  id: string;
  name: string;
}

interface ServiceListProps {
  services: Service[];
  handleChangePage: any;
  handleChangeRowsPerPage: any;
}

const ServiceList: React.FC<ServiceListProps> = ({ services }) => {
  return (
    <ul>
      {services.map((service) => (
        <li key={service.id}>{service.name}</li>
      ))}
    </ul>
  );
};

function CheckEmptyList({
  query,
  handleChangePage,
  handleChangeRowsPerPage,
}: {
  query: UseQueryResult<GetResult, Error>;
  handleChangePage: any;
  handleChangeRowsPerPage: any;
}) {
  const results = use(query.promise);
  if (results.data.length === 0) {
    return <Box>No services found</Box>;
  }
  const data = {
    items: results.data,
    paging: {
      totalPages: results.paging.total_pages,
      totalElements: results.paging.total_records,
      pageSize: results.paging.page_size,
      page: results.paging.current_page,
    },
  };
  return (
    <ServiceList
      services={data.items}
      handleChangePage={handleChangePage}
      handleChangeRowsPerPage={handleChangeRowsPerPage}
    />
  );
}

const ServiceListController = () => {
  const api = useAPIClient();

  const [paging, setPaging] = useState<any>({
    totalPages: 0,
    totalElements: 0,
    pageSize: 5,
    page: 0,
  });

  const page = paging.page;
  const pageSize = paging.pageSize;

  const query = api.get(
    ["list-services", paging],
    `/api/activity?page=${page}&recordsPerPage=${pageSize}`
  );

  const _handleService = (page: number, pageSize: number) => {
    setPaging({ ...paging, ...{ page, pageSize } });
  };

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <CheckEmptyList
        query={query}
        handleChangePage={(e: any, page: number) => {
          _handleService(page, paging.pageSize);
        }}
        handleChangeRowsPerPage={(e: any) => {
          _handleService(paging.page, e.target.value);
        }}
      />
    </Suspense>
  );
};

export default ServiceListController;
