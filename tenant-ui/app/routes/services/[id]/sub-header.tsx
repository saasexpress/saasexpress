import { Grid2, Link, Stack, Typography } from "@mui/material";
import AccountTreeIcon from '@mui/icons-material/AccountTree';

interface SubHeaderProps {
  serviceId: string;
  variantId: String;
  variants?: any;
}

export default function SubHeader({ serviceId, variantId, variants }: SubHeaderProps) {

  return (
      <Grid2
        container
        direction="row"
        alignItems="top"
        p={2}
        mt={2}
        mb={2}
        spacing={{ xs: 1, md: 1 }}
        columns={{ xs: 1, sm: 8, md: 12 }}
        sx={{ backgroundColor: "rgb(237, 237, 237)" }}
      >
        {Object.values(variants).sort((v1: any,v2:any) => v1.dag.name.localeCompare(v2.dag.name)).map((variant: any) => (
            <Grid2 sx={{backgroundColor: variant.dag.name == variantId ? "lightblue":"white", p:1}} size={{ xs: 1, sm: 4, md: 4 }}>
              <Stack direction="row" alignItems="center" justifyContent="left">
                <AccountTreeIcon />
                <Link href={`/ui/services/${serviceId}/editor?variant=${variant.dag.name}`}>

                <Typography paddingLeft={1}>
                  {variant.dag.name}
                </Typography>
                </Link>
              </Stack>
            </Grid2>
        ))}
      </Grid2>
  )
}
