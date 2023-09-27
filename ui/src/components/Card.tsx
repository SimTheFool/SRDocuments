import { Box, Heading, Card as RadCard } from "@radix-ui/themes";
import React from "react";
import styles from "./Card.module.css";

type CardProps = {
  children?: React.ReactNode;
  title?: React.ReactNode;
  note?: React.ReactNode;
};

export const Card = ({ title, children, note }: CardProps) => {
  return (
    <RadCard className={styles.card}>
      {title && (
        <Box className={styles.cardTitle} px={"1"}>
          <Heading size={"1"} as={"h3"} weight={"light"}>
            {title}
          </Heading>
        </Box>
      )}

      {note && (
        <Box className={styles.cardNote} px={"1"} asChild>
          <Heading size={"1"} as={"h3"} weight={"light"}>
            {note}
          </Heading>
        </Box>
      )}
      {children}
    </RadCard>
  );
};
