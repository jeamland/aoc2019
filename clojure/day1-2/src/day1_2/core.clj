(ns day1-2.core
  (:gen-class))

(defn fuel_for_mass
  [mass]
  (if (< mass 9) 0 (- (int (Math/floor (/ mass 3))) 2)))

(defn total_fuel_for_mass
  [mass]
  (apply + (rest (take-while pos? (iterate fuel_for_mass mass)))))

(defn -main
  [& _args]
  (println "Total:" (apply + (map total_fuel_for_mass (map #(Integer/parseInt %) (line-seq (java.io.BufferedReader. *in*)))))))