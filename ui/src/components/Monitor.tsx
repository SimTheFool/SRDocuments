import { Box, Grid, Heading } from "@radix-ui/themes";
import { ReactNode } from "react";
import { FaSkull } from "react-icons/fa";
import styles from "./Monitor.module.css";

type MonitorProps = {
  hit: number;
  overHit?: number;
  title: ReactNode;
};

export const Monitor = ({ hit, title, overHit = 0 }: MonitorProps) => {
  const xx = overHit % 4;
  const yy = (overHit - xx) / 4;
  const overHitBox = Array.from({ length: yy }).map(() => 4);
  overHitBox.push(xx);

  return (
    <Box>
      <Heading
        size={"1"}
        as={"h3"}
        style={{
          display: "block",
          whiteSpace: "nowrap",
        }}
      >
        {title}
      </Heading>
      <Grid columns={"3"} gap="0">
        {Array.from({ length: hit }).map(() => (
          <HitBox />
        ))}
        {overHitBox.map((n) => (
          <OverHitBox n={n} />
        ))}
      </Grid>
    </Box>
  );
};

const HitBox = ({}: {}) => {
  return (
    <Box className={styles.box}>
      <Box className={styles.incurable}>
        <FaSkull />
      </Box>
    </Box>
  );
};

const OverHitBox = ({ n }: { n: number }) => {
  return (
    <Grid columns={"2"} rows={"2"} gap="0">
      {Array.from({ length: n }).map(() => (
        <Box className={styles.overbox}></Box>
      ))}
    </Grid>
  );
};
