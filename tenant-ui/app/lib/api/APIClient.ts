import {
  keepPreviousData as keepPreviousDataFn,
  useQuery,
  UseQueryResult,
} from "@tanstack/react-query";
import APICall from "@lib/api/APICall";

interface APIClientProps {
  throwError?: boolean;
  refetchOnMount?: boolean;
  keepPreviousData?: boolean;
}

export interface GetResult {
  data: any;
  paging: any;
  isLoading: boolean;
}

const caller = APICall();

const useAPIClient = () => {
  //const { session } = useAppContext();

  return {
    post: caller.post,
    put: caller.put,
    delete: caller.delete,
    get: (
      queryKey: string[],
      endpoint: string,
      options: APIClientProps = {
        throwError: true,
        refetchOnMount: false,
        keepPreviousData: true,
      }
    ): UseQueryResult<GetResult, Error> => {
      return useQuery({
        queryKey,
        queryFn: async () => {
          return new Promise((resolve, reject) => {
            caller.get(
              endpoint,
              (data: any, paging: any) => {
                resolve({ data, paging } as any);
              },
              options.throwError
                ? (((er: any) => {
                    reject(er);
                  }) as any)
                : undefined
            );
          });
        },
        retry: false,
        refetchOnMount: options.refetchOnMount,
        placeholderData: options.keepPreviousData
          ? keepPreviousDataFn
          : undefined,
      });
    },
  };
};

export default useAPIClient;
