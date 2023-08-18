import { Box, Heading, Inset, Card as RadCard, Text } from "@radix-ui/themes";
import styles from "./Card.module.css";
import React from "react";

type CardProps = {
  children?: React.ReactNode;
  title?: React.ReactNode;
};

export const Card = ({ title, children }: CardProps) => {
  return (
    <RadCard className={styles.card}>
      <Box className={styles.cardTitle} px={"1"} asChild>
        <Heading size={"1"} as={"h3"} weight={"light"}>
          {title}
        </Heading>
      </Box>
      {children}
    </RadCard>
  );
};
