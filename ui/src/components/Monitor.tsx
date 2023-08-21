import { Box, Flex, Grid, Heading } from "@radix-ui/themes";
import styles from "./Monitor.module.css";
import { ReactNode } from "react";

type MonitorProps = {
  hit: number;
  title: ReactNode;
  inline?: boolean;
};

export const Monitor = ({ hit, title }: MonitorProps) => {
  return (
    <Box>
      <Heading
        size={"1"}
        as={"h3"}
        style={{
          display: "block",
        }}
      >
        {title}
      </Heading>
      <Grid columns={"3"} gap="0">
        {Array.from({ length: hit }).map(() => (
          <Box className={styles.box} />
        ))}
      </Grid>
    </Box>
  );
};

export const InlineMonitor = ({ hit, title }: MonitorProps) => {
  return (
    <Box>
      <Heading
        size={"1"}
        as={"h3"}
        style={{
          display: "block",
        }}
      >
        {title}
      </Heading>
      <Grid columns={"10"} gap="0">
        {Array.from({ length: hit }).map(() => (
          <Box className={styles.inlineBox} />
        ))}
      </Grid>
    </Box>
  );
};
